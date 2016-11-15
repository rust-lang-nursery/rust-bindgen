//! Intermediate representation of variables.

use cexpr;
use clang;
use parse::{ClangItemParser, ClangSubItemParser, ParseError, ParseResult};
use std::num::Wrapping;
use super::context::BindgenContext;
use super::function::cursor_mangling;
use super::int::IntKind;
use super::item::{Item, ItemId};
use super::ty::TypeKind;

/// A `Var` is our intermediate representation of a variable.
#[derive(Debug)]
pub struct Var {
    /// The name of the variable.
    name: String,
    /// The mangled name of the variable.
    mangled_name: Option<String>,
    /// The type of the variable.
    ty: ItemId,
    /// TODO: support non-integer constants?
    /// The integer value of the variable.
    val: Option<i64>,
    /// Whether this variable is const.
    is_const: bool,
}

impl Var {
    /// Construct a new `Var`.
    pub fn new(name: String,
               mangled: Option<String>,
               ty: ItemId,
               val: Option<i64>,
               is_const: bool)
               -> Var {
        assert!(!name.is_empty());
        Var {
            name: name,
            mangled_name: mangled,
            ty: ty,
            val: val,
            is_const: is_const,
        }
    }

    /// Is this variable `const` qualified?
    pub fn is_const(&self) -> bool {
        self.is_const
    }

    /// The value of this constant variable, if any.
    pub fn val(&self) -> Option<i64> {
        self.val
    }

    /// Get this variable's type.
    pub fn ty(&self) -> ItemId {
        self.ty
    }

    /// Get this variable's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get this variable's mangled name.
    pub fn mangled_name(&self) -> Option<&str> {
        self.mangled_name.as_ref().map(|n| &**n)
    }
}

impl ClangSubItemParser for Var {
    fn parse(cursor: clang::Cursor,
             ctx: &mut BindgenContext)
             -> Result<ParseResult<Self>, ParseError> {
        use clangll::*;
        use cexpr::expr::EvalResult;
        match cursor.kind() {
            CXCursor_MacroDefinition => {
                let value = parse_macro(ctx, &cursor, ctx.translation_unit());

                let (id, value) = match value {
                    Some(v) => v,
                    None => return Err(ParseError::Continue),
                };

                assert!(!id.is_empty(), "Empty macro name?");

                let previously_defined = ctx.parsed_macro(&id);

                // NB: It's important to "note" the macro even if the result is
                // not an integer, otherwise we might loose other kind of
                // derived macros.
                ctx.note_parsed_macro(id.clone(), value.clone());

                if previously_defined {
                    let name = String::from_utf8(id).unwrap();
                    warn!("Duplicated macro definition: {}", name);
                    return Err(ParseError::Continue);
                }

                // NOTE: Unwrapping, here and above, is safe, because the
                // identifier of a token comes straight from clang, and we
                // enforce utf8 there, so we should have already panicked at
                // this point.
                let name = String::from_utf8(id).unwrap();
                let (int_kind, val) = match value {
                    // TODO(emilio): Handle the non-invalid ones!
                    EvalResult::Float(..) |
                    EvalResult::Char(..) |
                    EvalResult::Str(..) |
                    EvalResult::Invalid => return Err(ParseError::Continue),

                    EvalResult::Int(Wrapping(value)) => {
                        let kind = ctx.options()
                            .type_chooser
                            .as_ref()
                            .and_then(|c| c.int_macro(&name, value))
                            .unwrap_or_else(|| {
                                if value < 0 {
                                    if value < i32::min_value() as i64 {
                                        IntKind::LongLong
                                    } else {
                                        IntKind::Int
                                    }
                                } else if value > u32::max_value() as i64 {
                                    IntKind::ULongLong
                                } else {
                                    IntKind::UInt
                                }
                            });

                        (kind, value)
                    }
                };

                let ty = Item::builtin_type(TypeKind::Int(int_kind), true, ctx);

                Ok(ParseResult::New(Var::new(name, None, ty, Some(val), true),
                                    Some(cursor)))
            }
            CXCursor_VarDecl => {
                let name = cursor.spelling();
                if name.is_empty() {
                    warn!("Empty constant name?");
                    return Err(ParseError::Continue);
                }

                let ty = cursor.cur_type();

                // XXX this is redundant, remove!
                let is_const = ty.is_const();

                let ty = Item::from_ty(&ty, Some(cursor), None, ctx)
                    .expect("Unable to resolve constant type?");

                // Note: Ty might not be totally resolved yet, see
                // tests/headers/inner_const.hpp
                //
                // That's fine because in that case we know it's not a literal.
                let is_integer = ctx.safe_resolve_type(ty)
                    .and_then(|t| t.safe_canonical_type(ctx))
                    .map(|t| t.is_integer())
                    .unwrap_or(false);

                let value = if is_integer {
                    cursor.evaluate()
                        .as_int()
                        .map(|val| val as i64)
                        .or_else(|| {
                            let tu = ctx.translation_unit();
                            get_integer_literal_from_cursor(&cursor, tu)
                        })
                } else {
                    None
                };


                let mangling = cursor_mangling(&cursor);

                let var = Var::new(name, mangling, ty, value, is_const);
                Ok(ParseResult::New(var, Some(cursor)))

            }
            _ => {
                /* TODO */
                Err(ParseError::Continue)
            }
        }
    }
}

/// Try and parse a macro using all the macros parsed until now.
fn parse_macro(ctx: &BindgenContext,
               cursor: &clang::Cursor,
               unit: &clang::TranslationUnit)
               -> Option<(Vec<u8>, cexpr::expr::EvalResult)> {
    use cexpr::{expr, nom};

    let cexpr_tokens = match unit.cexpr_tokens(cursor) {
        None => return None,
        Some(tokens) => tokens,
    };

    let parser = expr::IdentifierParser::new(ctx.parsed_macros());
    let result = parser.macro_definition(&cexpr_tokens);

    match result {
        nom::IResult::Done(_, (id, val)) => Some((id.into(), val)),
        _ => None,
    }
}

fn parse_int_literal_tokens(cursor: &clang::Cursor,
                            unit: &clang::TranslationUnit)
                            -> Option<i64> {
    use cexpr::{expr, nom};
    use cexpr::expr::EvalResult;

    let cexpr_tokens = match unit.cexpr_tokens(cursor) {
        None => return None,
        Some(tokens) => tokens,
    };

    // TODO(emilio): We can try to parse other kinds of literals.
    match expr::expr(&cexpr_tokens) {
        nom::IResult::Done(_, EvalResult::Int(Wrapping(val))) => Some(val),
        _ => None,
    }
}

fn get_integer_literal_from_cursor(cursor: &clang::Cursor,
                                   unit: &clang::TranslationUnit)
                                   -> Option<i64> {
    use clangll::*;
    let mut value = None;
    cursor.visit(|c| {
        match c.kind() {
            CXCursor_IntegerLiteral |
            CXCursor_UnaryOperator => {
                value = parse_int_literal_tokens(&c, unit);
            }
            CXCursor_UnexposedExpr => {
                value = get_integer_literal_from_cursor(&c, unit);
            }
            _ => (),
        }
        if value.is_some() {
            CXChildVisit_Break
        } else {
            CXChildVisit_Continue
        }
    });
    value
}

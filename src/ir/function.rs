use super::item::{Item, ItemId};
use super::ty::TypeKind;
use super::context::BindgenContext;
use syntax::abi;
use clang;
use clangll::Enum_CXCallingConv;
use parse::{ClangItemParser, ClangSubItemParser, ParseError, ParseResult};

/// A function declaration , with a signature, arguments, and argument names.
///
/// The argument names vector must be the same length as the ones in the
/// signature.
#[derive(Debug)]
pub struct Function {
    name: String,
    /// The mangled name, that is, the symbol.
    mangled_name: Option<String>,
    /// The id pointing to the current function signature.
    signature: ItemId,
    /// The doc comment on the function, if any.
    comment: Option<String>,
}

impl Function {
    pub fn new(name: String,
               mangled_name: Option<String>,
               sig: ItemId,
               comment: Option<String>) -> Self {
        Function {
            name: name,
            mangled_name: mangled_name,
            signature: sig,
            comment: comment,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn mangled_name(&self) -> Option<&str> {
        self.mangled_name.as_ref().map(|n| &**n)
    }

    pub fn signature(&self) -> ItemId {
        self.signature
    }
}

/// A function signature.
#[derive(Debug)]
pub struct FunctionSig {
    /// The return type of the function.
    return_type: ItemId,
    /// The type of the arguments, optionally with the name of the argument when
    /// declared.
    argument_types: Vec<(Option<String>, ItemId)>,
    /// Whether this function is variadic.
    is_variadic: bool,
    /// The abi of this function.
    abi: abi::Abi,
}

fn get_abi(cc: Enum_CXCallingConv) -> abi::Abi {
    use clangll::*;
    match cc {
        CXCallingConv_Default => abi::Abi::C,
        CXCallingConv_C => abi::Abi::C,
        CXCallingConv_X86StdCall => abi::Abi::Stdcall,
        CXCallingConv_X86FastCall => abi::Abi::Fastcall,
        CXCallingConv_AAPCS => abi::Abi::Aapcs,
        CXCallingConv_X86_64Win64 => abi::Abi::Win64,
        other => panic!("unsupported calling convention: {}", other),
    }
}

pub fn cursor_mangling(cursor: &clang::Cursor) -> Option<String> {
    // We early return here because libclang may crash in some case
    // if we pass in a variable inside a partial specialized template.
    // See servo/rust-bindgen#67.
    if cursor.is_in_non_fully_specialized_template() {
        return None;
    }

    let mut mangling = cursor.mangling();

    // Try to undo backend linkage munging (prepended _, generally)
    if cfg!(target_os = "macos") {
        mangling.remove(0);
    }

    if mangling.is_empty() { None } else { Some(mangling) }
}

impl FunctionSig {
    pub fn new(return_type: ItemId,
               arguments: Vec<(Option<String>, ItemId)>,
               is_variadic: bool,
               abi: abi::Abi) -> Self {
        FunctionSig {
            return_type: return_type,
            argument_types: arguments,
            is_variadic: is_variadic,
            abi: abi,
        }
    }

    pub fn from_ty(ty: &clang::Type,
                   cursor: &clang::Cursor,
                   ctx: &mut BindgenContext) -> Result<Self, ParseError> {
        use clangll::*;
        debug!("FunctionSig::from_ty {:?} {:?}", ty, cursor);

        if cursor.is_inlined_function() && !ctx.options().keep_inline_functions {
            return Err(ParseError::Continue);
        }

        // Don't parse operatorxx functions in C++
        let spelling = cursor.spelling();
        if spelling.starts_with("operator") {
            return Err(ParseError::Continue);
        }

        let cursor = if cursor.is_valid() {
            *cursor
        } else {
            ty.declaration()
        };

        let mut args: Vec<_> = match cursor.kind() {
            CXCursor_FunctionDecl |
            CXCursor_CXXMethod => {
                // For CXCursor_FunctionDecl, cursor.args() is the reliable way
                // to get parameter names and types.
                cursor.args().iter().map(|arg| {
                    let arg_ty = arg.cur_type();
                    let name = arg.spelling();
                    let name = if name.is_empty() { None } else { Some(name) };
                    let ty = Item::from_ty(&arg_ty, Some(*arg), None, ctx)
                                    .expect("Argument?");
                    (name, ty)
                }).collect()
            }
            _ => {
                // For non-CXCursor_FunctionDecl, visiting the cursor's children
                // is the only reliable way to get parameter names.
                let mut args = vec![];
                cursor.visit(|c, _| {
                    if c.kind() == CXCursor_ParmDecl {
                        let ty = Item::from_ty(&c.cur_type(), Some(*c), None, ctx)
                                    .expect("ParmDecl?");
                        let name = c.spelling();
                        let name = if name.is_empty() { None } else { Some(name) };
                        args.push((name, ty));
                    }
                    CXChildVisit_Continue
                });
                args
            }
        };

        if cursor.kind() == CXCursor_CXXMethod {
            let is_const = cursor.method_is_const();
            let is_virtual = cursor.method_is_virtual();
            let is_static = cursor.method_is_static();
            if !is_static && !is_virtual {
                let class = Item::parse(cursor.semantic_parent(), None, ctx)
                                .expect("Expected to parse the class");
                let ptr = Item::builtin_type(TypeKind::Pointer(class), is_const, ctx);
                args.insert(0, (Some("this".into()), ptr));
            } else if is_virtual {
                let void = Item::builtin_type(TypeKind::Void, false, ctx);
                let ptr = Item::builtin_type(TypeKind::Pointer(void), false, ctx);
                args.insert(0, (Some("this".into()), ptr));
            }
        }

        let ret = try!(Item::from_ty(&ty.ret_type(), None, None, ctx));
        let abi = get_abi(ty.call_conv());

        Ok(Self::new(ret, args, ty.is_variadic(), abi))
    }

    pub fn return_type(&self) -> ItemId {
        self.return_type
    }

    pub fn argument_types(&self) -> &[(Option<String>, ItemId)] {
        &self.argument_types
    }

    pub fn abi(&self) -> abi::Abi {
        self.abi
    }

    pub fn is_variadic(&self) -> bool {
        // Clang reports some functions as variadic when they *might* be
        // variadic. We do the argument check because rust doesn't codegen well
        // variadic functions without an initial argument.
        self.is_variadic && !self.argument_types.is_empty()
    }
}

impl ClangSubItemParser for Function {
    fn parse(cursor: clang::Cursor,
             context: &mut BindgenContext) -> Result<ParseResult<Self>, ParseError> {
        use clangll::*;
        match cursor.kind() {
            CXCursor_FunctionDecl |
            CXCursor_CXXMethod => {},
            _ => return Err(ParseError::Continue),
        };

        debug!("Function::parse({:?}, {:?})", cursor, cursor.cur_type());

        // Grab the signature using Item::from_ty.
        let sig = try!(Item::from_ty(&cursor.cur_type(), Some(cursor), None, context));

        let name = cursor.spelling();
        assert!(!name.is_empty(), "Empty function name?");

        let mut mangled_name = cursor_mangling(&cursor);
        if mangled_name.as_ref() == Some(&name) {
            mangled_name = None;
        }

        let comment = cursor.raw_comment();

        let function = Self::new(name, mangled_name, sig, comment);
        Ok(ParseResult::New(function, Some(cursor)))
    }
}

use crate::ir::function::Abi;
use proc_macro2::Ident;

/// Used to build the output tokens for dynamic bindings.
pub struct DynamicItems {
    /// Tracks the tokens that will appears inside the library struct -- e.g.:
    /// ```ignore
    /// struct Lib {
    ///    __library: ::libloading::Library,
    ///    x: Result<unsafe extern ..., ::libloading::Error>, // <- tracks these
    ///    ...
    /// }
    /// ```
    struct_members: Vec<proc_macro2::TokenStream>,

    /// Tracks the tokens that will appear inside the library struct's implementation, e.g.:
    ///
    /// ```ignore
    /// impl Lib {
    ///     ...
    ///     pub unsafe fn foo(&self, ...) { // <- tracks these
    ///         ...
    ///     }
    /// }
    /// ```
    struct_implementation: Vec<proc_macro2::TokenStream>,

    /// Tracks the tokens that will appear inside the struct used for checking if a symbol is
    /// usable, e.g.:
    /// ```ignore
    /// pub fn f(&self) -> Result<(), &'a ::libloading::Error> { // <- tracks these
    ///     self.__library.f.as_ref().map(|_| ())
    /// }
    /// ```
    runtime_checks: Vec<proc_macro2::TokenStream>,

    /// Tracks the initialization of the fields inside the `::new` constructor of the library
    /// struct, e.g.:
    /// ```ignore
    /// impl Lib {
    ///
    ///     pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    ///     where
    ///         P: AsRef<::std::ffi::OsStr>,
    ///     {
    ///         ...
    ///         let foo = __library.get(...) ...; // <- tracks these
    ///         ...
    ///     }
    ///
    ///     ...
    /// }
    /// ```
    constructor_inits: Vec<proc_macro2::TokenStream>,

    /// Tracks the information that is passed to the library struct at the end of the `::new`
    /// constructor, e.g.:
    /// ```ignore
    /// impl LibFoo {
    ///     pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    ///     where
    ///         P: AsRef<::std::ffi::OsStr>,
    ///     {
    ///         ...
    ///         Ok(LibFoo {
    ///             __library: __library,
    ///             foo,
    ///             bar, // <- tracks these
    ///             ...
    ///         })
    ///     }
    /// }
    /// ```
    init_fields: Vec<proc_macro2::TokenStream>,
}

impl Default for DynamicItems {
    fn default() -> Self {
        DynamicItems {
            struct_members: vec![],
            struct_implementation: vec![],
            runtime_checks: vec![],
            constructor_inits: vec![],
            init_fields: vec![],
        }
    }
}

impl DynamicItems {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_tokens(
        &self,
        lib_ident: Ident,
        check_struct_ident: Ident,
    ) -> proc_macro2::TokenStream {
        let struct_members = &self.struct_members;
        let constructor_inits = &self.constructor_inits;
        let init_fields = &self.init_fields;
        let struct_implementation = &self.struct_implementation;
        let runtime_checks = &self.runtime_checks;
        quote! {
            extern crate libloading;

            pub struct #lib_ident {
                __library: ::libloading::Library,
                #(#struct_members)*
            }

            impl #lib_ident {
                pub unsafe fn new<P>(
                    path: P
                ) -> Result<Self, ::libloading::Error>
                where P: AsRef<::std::ffi::OsStr> {
                    let __library = ::libloading::Library::new(path)?;
                    #( #constructor_inits )*
                    Ok(
                        #lib_ident {
                            __library: __library,
                            #( #init_fields ),*
                        }
                    )
                }

                pub fn can_call(&self) -> #check_struct_ident {
                    #check_struct_ident { __library: self }
                }

                #( #struct_implementation )*
            }

            pub struct #check_struct_ident<'a> {
                __library: &'a #lib_ident,
            }

            impl<'a> #check_struct_ident<'a> {
                #( #runtime_checks )*
            }
        }
    }

    pub fn add_function(
        &mut self,
        ident: Ident,
        abi: Abi,
        args: Vec<proc_macro2::TokenStream>,
        args_identifiers: Vec<proc_macro2::TokenStream>,
        ret: proc_macro2::TokenStream,
        ret_ty: proc_macro2::TokenStream,
    ) {
        assert_eq!(args.len(), args_identifiers.len());

        self.struct_members.push(quote!{
            #ident: Result<unsafe extern #abi fn ( #( #args ),* ) #ret, ::libloading::Error>,
        });

        self.struct_implementation.push(quote! {
            pub unsafe fn #ident ( &self, #( #args ),* ) -> #ret_ty {
                let sym = self.#ident.as_ref().expect("Expected function, got error.");
                (sym)(#( #args_identifiers ),*)
            }
        });

        self.runtime_checks.push(quote! {
            pub fn #ident (&self) -> Result<(), &'a::libloading::Error> {
                self.__library.#ident.as_ref().map(|_| ())
            }
        });

        let ident_str = ident.to_string();
        self.constructor_inits.push(quote! {
            let #ident = __library.get(#ident_str.as_bytes()).map(|sym| *sym);
        });

        self.init_fields.push(quote! {
            #ident
        });
    }
}

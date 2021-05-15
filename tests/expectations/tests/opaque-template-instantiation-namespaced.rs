#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod zoidberg {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct Template<T> {
            pub member: T,
            pub _phantom_0:
                ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
        }
        impl<T> Default for Template<T> {
            fn default() -> Self {
                let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
                unsafe {
                    ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                    s.assume_init()
                }
            }
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct Foo {
            pub c: ::std::os::raw::c_char,
        }
        #[test]
        fn bindgen_test_layout_Foo() {
            assert_eq!(
                ::std::mem::size_of::<Foo>(),
                1usize,
                concat!("Size of: ", stringify!(Foo))
            );
            assert_eq!(
                ::std::mem::align_of::<Foo>(),
                1usize,
                concat!("Alignment of ", stringify!(Foo))
            );
            assert_eq!(
                {
                    const STRUCT_SIZE: usize = std::mem::size_of::<Foo>();
                    let buffer = [0u8; STRUCT_SIZE];
                    let struct_instance = unsafe {
                        std::mem::transmute::<[u8; STRUCT_SIZE], Foo>(buffer)
                    };
                    let struct_ptr = &struct_instance as *const Foo;
                    let field_ptr = std::ptr::addr_of!(struct_instance.c);
                    let struct_address = struct_ptr as usize;
                    let field_address = field_ptr as usize;
                    std::mem::forget(struct_instance);
                    field_address.checked_sub(struct_address).unwrap()
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(Foo),
                    "::",
                    stringify!(c)
                )
            );
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct Bar {
            pub i: ::std::os::raw::c_int,
        }
        #[test]
        fn bindgen_test_layout_Bar() {
            assert_eq!(
                ::std::mem::size_of::<Bar>(),
                4usize,
                concat!("Size of: ", stringify!(Bar))
            );
            assert_eq!(
                ::std::mem::align_of::<Bar>(),
                4usize,
                concat!("Alignment of ", stringify!(Bar))
            );
            assert_eq!(
                {
                    const STRUCT_SIZE: usize = std::mem::size_of::<Bar>();
                    let buffer = [0u8; STRUCT_SIZE];
                    let struct_instance = unsafe {
                        std::mem::transmute::<[u8; STRUCT_SIZE], Bar>(buffer)
                    };
                    let struct_ptr = &struct_instance as *const Bar;
                    let field_ptr = std::ptr::addr_of!(struct_instance.i);
                    let struct_address = struct_ptr as usize;
                    let field_address = field_ptr as usize;
                    std::mem::forget(struct_instance);
                    field_address.checked_sub(struct_address).unwrap()
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(Bar),
                    "::",
                    stringify!(i)
                )
            );
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct ContainsInstantiation {
            pub not_opaque: root::zoidberg::Template<root::zoidberg::Foo>,
        }
        #[test]
        fn bindgen_test_layout_ContainsInstantiation() {
            assert_eq!(
                ::std::mem::size_of::<ContainsInstantiation>(),
                1usize,
                concat!("Size of: ", stringify!(ContainsInstantiation))
            );
            assert_eq!(
                ::std::mem::align_of::<ContainsInstantiation>(),
                1usize,
                concat!("Alignment of ", stringify!(ContainsInstantiation))
            );
            assert_eq!(
                {
                    const STRUCT_SIZE: usize =
                        std::mem::size_of::<ContainsInstantiation>();
                    let buffer = [0u8; STRUCT_SIZE];
                    let struct_instance = unsafe {
                        std::mem::transmute::<
                            [u8; STRUCT_SIZE],
                            ContainsInstantiation,
                        >(buffer)
                    };
                    let struct_ptr =
                        &struct_instance as *const ContainsInstantiation;
                    let field_ptr =
                        std::ptr::addr_of!(struct_instance.not_opaque);
                    let struct_address = struct_ptr as usize;
                    let field_address = field_ptr as usize;
                    std::mem::forget(struct_instance);
                    field_address.checked_sub(struct_address).unwrap()
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(ContainsInstantiation),
                    "::",
                    stringify!(not_opaque)
                )
            );
        }
        impl Default for ContainsInstantiation {
            fn default() -> Self {
                let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
                unsafe {
                    ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                    s.assume_init()
                }
            }
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct ContainsOpaqueInstantiation {
            pub opaque: u32,
        }
        #[test]
        fn bindgen_test_layout_ContainsOpaqueInstantiation() {
            assert_eq!(
                ::std::mem::size_of::<ContainsOpaqueInstantiation>(),
                4usize,
                concat!("Size of: ", stringify!(ContainsOpaqueInstantiation))
            );
            assert_eq!(
                ::std::mem::align_of::<ContainsOpaqueInstantiation>(),
                4usize,
                concat!(
                    "Alignment of ",
                    stringify!(ContainsOpaqueInstantiation)
                )
            );
            assert_eq!(
                {
                    const STRUCT_SIZE: usize =
                        std::mem::size_of::<ContainsOpaqueInstantiation>();
                    let buffer = [0u8; STRUCT_SIZE];
                    let struct_instance = unsafe {
                        std::mem::transmute::<
                            [u8; STRUCT_SIZE],
                            ContainsOpaqueInstantiation,
                        >(buffer)
                    };
                    let struct_ptr =
                        &struct_instance as *const ContainsOpaqueInstantiation;
                    let field_ptr = std::ptr::addr_of!(struct_instance.opaque);
                    let struct_address = struct_ptr as usize;
                    let field_address = field_ptr as usize;
                    std::mem::forget(struct_instance);
                    field_address.checked_sub(struct_address).unwrap()
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(ContainsOpaqueInstantiation),
                    "::",
                    stringify!(opaque)
                )
            );
        }
    }
    #[test]
    fn __bindgen_test_layout_Template_open0_Foo_close0_instantiation() {
        assert_eq!(
            ::std::mem::size_of::<root::zoidberg::Template<root::zoidberg::Foo>>(
            ),
            1usize,
            concat!(
                "Size of template specialization: ",
                stringify!(root::zoidberg::Template<root::zoidberg::Foo>)
            )
        );
        assert_eq!(
            ::std::mem::align_of::<root::zoidberg::Template<root::zoidberg::Foo>>(
            ),
            1usize,
            concat!(
                "Alignment of template specialization: ",
                stringify!(root::zoidberg::Template<root::zoidberg::Foo>)
            )
        );
    }
}

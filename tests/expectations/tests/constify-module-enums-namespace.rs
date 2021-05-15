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
    pub mod ns1 {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub mod ns2 {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            pub mod foo {
                pub type Type = ::std::os::raw::c_uint;
                pub const THIS: Type = 0;
                pub const SHOULD_BE: Type = 1;
                pub const A_CONSTANT: Type = 2;
            }
        }
        pub mod ns3 {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct bar {
                pub this_should_work: root::ns1::ns2::foo::Type,
            }
            #[test]
            fn bindgen_test_layout_bar() {
                assert_eq!(
                    ::std::mem::size_of::<bar>(),
                    4usize,
                    concat!("Size of: ", stringify!(bar))
                );
                assert_eq!(
                    ::std::mem::align_of::<bar>(),
                    4usize,
                    concat!("Alignment of ", stringify!(bar))
                );
                assert_eq!(
                    {
                        const STRUCT_SIZE: usize = std::mem::size_of::<bar>();
                        let buffer = [0u8; STRUCT_SIZE];
                        let struct_instance = unsafe {
                            std::mem::transmute::<[u8; STRUCT_SIZE], bar>(
                                buffer,
                            )
                        };
                        let struct_ptr = &struct_instance as *const bar;
                        let field_ptr = std::ptr::addr_of!(
                            struct_instance.this_should_work
                        );
                        let struct_address = struct_ptr as usize;
                        let field_address = field_ptr as usize;
                        std::mem::forget(struct_instance);
                        field_address.checked_sub(struct_address).unwrap()
                    },
                    0usize,
                    concat!(
                        "Offset of field: ",
                        stringify!(bar),
                        "::",
                        stringify!(this_should_work)
                    )
                );
            }
            impl Default for bar {
                fn default() -> Self {
                    let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
                    unsafe {
                        ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                        s.assume_init()
                    }
                }
            }
        }
    }
}

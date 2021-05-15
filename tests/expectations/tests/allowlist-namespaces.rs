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
    pub mod outer {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub mod inner {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            #[repr(C)]
            #[derive(Debug, Default, Copy, Clone)]
            pub struct Helper {
                pub _address: u8,
            }
            #[test]
            fn bindgen_test_layout_Helper() {
                assert_eq!(
                    ::std::mem::size_of::<Helper>(),
                    1usize,
                    concat!("Size of: ", stringify!(Helper))
                );
                assert_eq!(
                    ::std::mem::align_of::<Helper>(),
                    1usize,
                    concat!("Alignment of ", stringify!(Helper))
                );
            }
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Test {
            pub helper: root::outer::inner::Helper,
        }
        #[test]
        fn bindgen_test_layout_Test() {
            assert_eq!(
                ::std::mem::size_of::<Test>(),
                1usize,
                concat!("Size of: ", stringify!(Test))
            );
            assert_eq!(
                ::std::mem::align_of::<Test>(),
                1usize,
                concat!("Alignment of ", stringify!(Test))
            );
            assert_eq!(
                {
                    const STRUCT_SIZE: usize = std::mem::size_of::<Test>();
                    let buffer = [0u8; STRUCT_SIZE];
                    let struct_instance = unsafe {
                        std::mem::transmute::<[u8; STRUCT_SIZE], Test>(buffer)
                    };
                    let struct_ptr = &struct_instance as *const Test;
                    let field_ptr = std::ptr::addr_of!(struct_instance.helper);
                    let struct_address = struct_ptr as usize;
                    let field_address = field_ptr as usize;
                    std::mem::forget(struct_instance);
                    field_address.checked_sub(struct_address).unwrap()
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(Test),
                    "::",
                    stringify!(helper)
                )
            );
        }
    }
}

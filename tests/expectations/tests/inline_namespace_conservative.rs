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
    pub mod foo {
        #[allow(unused_imports)]
        use self::super::super::root;
        pub mod bar {
            #[allow(unused_imports)]
            use self::super::super::super::root;
            pub type Ty = ::std::os::raw::c_int;
        }
        pub type Ty = ::std::os::raw::c_longlong;
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Bar {
        pub baz: root::foo::bar::Ty,
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
                let field_ptr = std::ptr::addr_of!(struct_instance.baz);
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
                stringify!(baz)
            )
        );
    }
}

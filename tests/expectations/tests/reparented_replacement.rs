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
        /// <div rustbindgen replaces="foo::Bar"></div>
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone)]
        pub struct Bar {
            pub bazz: ::std::os::raw::c_int,
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
                unsafe {
                    &(*(::std::ptr::null::<Bar>())).bazz as *const _ as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(Bar),
                    "::",
                    stringify!(bazz)
                )
            );
        }
        struct Box_Bar {
            ptr: *mut ::std::ffi::c_void,
        }
        impl Box_Bar {}
        impl Drop for Box_Bar {
            fn drop(&mut self) {
                unsafe {
                    ::std::alloc::dealloc(
                        self.ptr as *mut u8,
                        ::std::alloc::Layout::from_size_align(4usize, 4usize)
                            .unwrap(),
                    );
                }
            }
        }
    }
    pub type ReferencesBar = root::foo::Bar;
}

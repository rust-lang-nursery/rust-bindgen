/* automatically generated by rust-bindgen */

pub use self::root::*;

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::*;
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct Foo {
        pub s: ::std::os::raw::c_int,
    }
    #[test]
    fn bindgen_test_layout_Foo() {
        assert_eq!(::std::mem::size_of::<Foo>() , 4usize , concat ! (
                   "Size of: " , stringify ! ( Foo ) ));
        assert_eq! (::std::mem::align_of::<Foo>() , 4usize , concat ! (
                    "Alignment of " , stringify ! ( Foo ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const Foo ) ) . s as * const _ as usize } ,
                    0usize , concat ! (
                    "Alignment of field: " , stringify ! ( Foo ) , "::" ,
                    stringify ! ( s ) ));
    }
    impl Clone for Foo {
        fn clone(&self) -> Self { *self }
    }
}

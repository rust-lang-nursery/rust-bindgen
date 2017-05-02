/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct nsRefPtrHashtable {
        pub _address: u8,
    }
    pub type nsRefPtrHashtable_UserDataType<PtrType> = *mut PtrType;
    impl Default for nsRefPtrHashtable {
        fn default() -> Self { unsafe { ::std::mem::zeroed() } }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct a {
        pub b: u8,
    }
    #[test]
    fn bindgen_test_layout_a() {
        assert_eq!(::std::mem::size_of::<a>() , 1usize , concat ! (
                   "Size of: " , stringify ! ( a ) ));
        assert_eq! (::std::mem::align_of::<a>() , 1usize , concat ! (
                    "Alignment of " , stringify ! ( a ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const a ) ) . b as * const _ as usize } ,
                    0usize , concat ! (
                    "Alignment of field: " , stringify ! ( a ) , "::" ,
                    stringify ! ( b ) ));
    }
    impl Clone for a {
        fn clone(&self) -> Self { *self }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct nsCSSValue {
        pub c: root::a,
    }
    #[test]
    fn bindgen_test_layout_nsCSSValue() {
        assert_eq!(::std::mem::size_of::<nsCSSValue>() , 1usize , concat ! (
                   "Size of: " , stringify ! ( nsCSSValue ) ));
        assert_eq! (::std::mem::align_of::<nsCSSValue>() , 1usize , concat ! (
                    "Alignment of " , stringify ! ( nsCSSValue ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const nsCSSValue ) ) . c as * const _ as
                    usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! ( nsCSSValue ) , "::"
                    , stringify ! ( c ) ));
    }
    impl Clone for nsCSSValue {
        fn clone(&self) -> Self { *self }
    }
}

/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct Foo {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(::std::mem::size_of::<Foo>() , 1usize , concat ! (
               "Size of: " , stringify ! ( Foo ) ));
    assert_eq! (::std::mem::align_of::<Foo>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( Foo ) ));
}
impl Clone for Foo {
    fn clone(&self) -> Self { *self }
}
impl Foo {
    #[inline]
    pub unsafe fn new(a: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::uninitialized();
        Foo_Foo(&mut __bindgen_tmp, a);
        __bindgen_tmp
    }
}

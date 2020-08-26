#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct foo {
    pub member: foo__bindgen_ty_1,
}
pub const foo_FOO_A: ::std::os::raw::c_uint = 0;
pub const foo_FOO_B: ::std::os::raw::c_uint = 1;
pub type foo__bindgen_ty_1 = ::std::os::raw::c_uint;
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        4usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        4usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<foo>())).member as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo),
            "::",
            stringify!(member)
        )
    );
}
impl Default for foo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub const Foo_Bar: Foo = 0;
pub const Foo_Qux: Foo = 1;
pub type Foo = ::std::os::raw::c_uint;
pub mod Neg {
    pub type Type = ::std::os::raw::c_int;
    pub const MinusOne: Type = -1;
    pub const One: Type = 1;
}

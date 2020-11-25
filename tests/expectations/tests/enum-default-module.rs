#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct foo {
    pub member: foo__bindgen_ty_1::Type,
}
pub mod foo__bindgen_ty_1 {
    pub type Type = ::std::os::raw::c_uint;
    pub const FOO_A: Type = 0;
    pub const FOO_B: Type = 1;
}
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
struct Box_foo {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_foo {}
impl Drop for Box_foo {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
pub mod Foo {
    pub type Type = ::std::os::raw::c_uint;
    pub const Bar: Type = 0;
    pub const Qux: Type = 1;
}
pub mod Neg {
    pub type Type = ::std::os::raw::c_int;
    pub const MinusOne: Type = -1;
    pub const One: Type = 1;
}
pub mod NoDebug {
    /// <div rustbindgen nodebug></div>
    pub type Type = ::std::os::raw::c_uint;
    pub const NoDebug1: Type = 0;
    pub const NoDebug2: Type = 1;
}

#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub union foo {
    pub bar: foo__bindgen_ty_1,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union foo__bindgen_ty_1 {
    pub a: ::std::os::raw::c_uint,
    pub b: ::std::os::raw::c_ushort,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<foo__bindgen_ty_1>())).a as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_1),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<foo__bindgen_ty_1>())).b as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo__bindgen_ty_1),
            "::",
            stringify!(b)
        )
    );
}
impl Default for foo__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_foo__bindgen_ty_1 {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_foo__bindgen_ty_1 {}
impl Drop for Box_foo__bindgen_ty_1 {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
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
        unsafe { &(*(::std::ptr::null::<foo>())).bar as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(bar))
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

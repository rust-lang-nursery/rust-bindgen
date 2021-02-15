#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

///```text
/// <div rustbindgen opaque></div>
///```
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OtherOpaque {
    pub _bindgen_opaque_blob: u32,
}
#[test]
fn bindgen_test_layout_OtherOpaque() {
    assert_eq!(
        ::std::mem::size_of::<OtherOpaque>(),
        4usize,
        concat!("Size of: ", stringify!(OtherOpaque))
    );
    assert_eq!(
        ::std::mem::align_of::<OtherOpaque>(),
        4usize,
        concat!("Alignment of ", stringify!(OtherOpaque))
    );
}
///```text
/// <div rustbindgen opaque></div>
///```
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Opaque {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq)]
pub struct WithOpaquePtr {
    pub whatever: *mut u8,
    pub other: u32,
    pub t: OtherOpaque,
}
#[test]
fn bindgen_test_layout_WithOpaquePtr() {
    assert_eq!(
        ::std::mem::size_of::<WithOpaquePtr>(),
        16usize,
        concat!("Size of: ", stringify!(WithOpaquePtr))
    );
    assert_eq!(
        ::std::mem::align_of::<WithOpaquePtr>(),
        8usize,
        concat!("Alignment of ", stringify!(WithOpaquePtr))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithOpaquePtr>())).whatever as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithOpaquePtr),
            "::",
            stringify!(whatever)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithOpaquePtr>())).other as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(WithOpaquePtr),
            "::",
            stringify!(other)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithOpaquePtr>())).t as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(WithOpaquePtr),
            "::",
            stringify!(t)
        )
    );
}
impl Default for WithOpaquePtr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// <div rustbindgen="true" replaces="whatever"></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct whatever {
    pub replacement: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_whatever() {
    assert_eq!(
        ::std::mem::size_of::<whatever>(),
        4usize,
        concat!("Size of: ", stringify!(whatever))
    );
    assert_eq!(
        ::std::mem::align_of::<whatever>(),
        4usize,
        concat!("Alignment of ", stringify!(whatever))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<whatever>())).replacement as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(whatever),
            "::",
            stringify!(replacement)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct container {
    pub c: whatever,
}
#[test]
fn bindgen_test_layout_container() {
    assert_eq!(
        ::std::mem::size_of::<container>(),
        4usize,
        concat!("Size of: ", stringify!(container))
    );
    assert_eq!(
        ::std::mem::align_of::<container>(),
        4usize,
        concat!("Alignment of ", stringify!(container))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<container>())).c as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(container),
            "::",
            stringify!(c)
        )
    );
}

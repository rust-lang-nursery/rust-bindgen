#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

///```text
/// Since builtin \`Clone\` impls were introduced in Rust 1.21 this struct
/// should impl \`Clone\` "manually".
///```
#[repr(C)]
#[derive(Copy)]
pub struct ShouldImplClone {
    pub large: [::std::os::raw::c_int; 33usize],
}
#[test]
fn bindgen_test_layout_ShouldImplClone() {
    assert_eq!(
        ::std::mem::size_of::<ShouldImplClone>(),
        132usize,
        concat!("Size of: ", stringify!(ShouldImplClone))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldImplClone>(),
        4usize,
        concat!("Alignment of ", stringify!(ShouldImplClone))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ShouldImplClone>())).large as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldImplClone),
            "::",
            stringify!(large)
        )
    );
}
impl Clone for ShouldImplClone {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ShouldImplClone {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

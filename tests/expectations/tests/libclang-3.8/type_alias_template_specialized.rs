#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Rooted {
    pub ptr: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Rooted() {
    assert_eq!(
        ::std::mem::size_of::<Rooted>(),
        4usize,
        concat!("Size of: ", stringify!(Rooted))
    );
    assert_eq!(
        ::std::mem::align_of::<Rooted>(),
        4usize,
        concat!("Alignment of ", stringify!(Rooted))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Rooted>())).ptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Rooted),
            "::",
            stringify!(ptr)
        )
    );
}
/// <div rustbindgen replaces="MaybeWrapped"></div>
pub type MaybeWrapped<a> = a;

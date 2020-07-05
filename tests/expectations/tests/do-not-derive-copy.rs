#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default)]
pub struct WouldBeCopyButWeAreNotDerivingCopy {
    pub x: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_WouldBeCopyButWeAreNotDerivingCopy() {
    assert_eq!(
        ::std::mem::size_of::<WouldBeCopyButWeAreNotDerivingCopy>(),
        4usize,
        concat!("Size of: ", stringify!(WouldBeCopyButWeAreNotDerivingCopy))
    );
    assert_eq!(
        ::std::mem::align_of::<WouldBeCopyButWeAreNotDerivingCopy>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(WouldBeCopyButWeAreNotDerivingCopy)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WouldBeCopyButWeAreNotDerivingCopy>())).x
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WouldBeCopyButWeAreNotDerivingCopy),
            "::",
            stringify!(x)
        )
    );
}

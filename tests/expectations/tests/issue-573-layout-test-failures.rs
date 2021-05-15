#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Outer {
    pub i: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AutoIdVector {
    pub ar: Outer,
}
#[test]
fn bindgen_test_layout_AutoIdVector() {
    assert_eq!(
        ::std::mem::size_of::<AutoIdVector>(),
        1usize,
        concat!("Size of: ", stringify!(AutoIdVector))
    );
    assert_eq!(
        ::std::mem::align_of::<AutoIdVector>(),
        1usize,
        concat!("Alignment of ", stringify!(AutoIdVector))
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<AutoIdVector>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], AutoIdVector>(buffer)
            };
            let struct_ptr = &struct_instance as *const AutoIdVector;
            let field_ptr = std::ptr::addr_of!(struct_instance.ar);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AutoIdVector),
            "::",
            stringify!(ar)
        )
    );
}
#[test]
fn __bindgen_test_layout_Outer_open0_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<Outer>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(Outer))
    );
    assert_eq!(
        ::std::mem::align_of::<Outer>(),
        1usize,
        concat!("Alignment of template specialization: ", stringify!(Outer))
    );
}

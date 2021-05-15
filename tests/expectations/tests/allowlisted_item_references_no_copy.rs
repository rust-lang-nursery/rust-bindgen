#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default)]
pub struct NoCopy {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_NoCopy() {
    assert_eq!(
        ::std::mem::size_of::<NoCopy>(),
        1usize,
        concat!("Size of: ", stringify!(NoCopy))
    );
    assert_eq!(
        ::std::mem::align_of::<NoCopy>(),
        1usize,
        concat!("Alignment of ", stringify!(NoCopy))
    );
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct AllowlistMe {
    pub a: NoCopy,
}
#[test]
fn bindgen_test_layout_AllowlistMe() {
    assert_eq!(
        ::std::mem::size_of::<AllowlistMe>(),
        1usize,
        concat!("Size of: ", stringify!(AllowlistMe))
    );
    assert_eq!(
        ::std::mem::align_of::<AllowlistMe>(),
        1usize,
        concat!("Alignment of ", stringify!(AllowlistMe))
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<AllowlistMe>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], AllowlistMe>(buffer)
            };
            let struct_ptr = &struct_instance as *const AllowlistMe;
            let field_ptr = std::ptr::addr_of!(struct_instance.a);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AllowlistMe),
            "::",
            stringify!(a)
        )
    );
}

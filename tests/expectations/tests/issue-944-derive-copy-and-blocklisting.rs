#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub struct BlocklistMe(u8);

/// Because this type contains a blocklisted type, it should not derive Copy.
#[repr(C)]
pub struct ShouldNotBeCopy {
    pub a: BlocklistMe,
}
#[test]
fn bindgen_test_layout_ShouldNotBeCopy() {
    assert_eq!(
        ::std::mem::size_of::<ShouldNotBeCopy>(),
        1usize,
        concat!("Size of: ", stringify!(ShouldNotBeCopy))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldNotBeCopy>(),
        1usize,
        concat!("Alignment of ", stringify!(ShouldNotBeCopy))
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<ShouldNotBeCopy>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], ShouldNotBeCopy>(
                    buffer,
                )
            };
            let struct_ptr = &struct_instance as *const ShouldNotBeCopy;
            let field_ptr = std::ptr::addr_of!(struct_instance.a);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldNotBeCopy),
            "::",
            stringify!(a)
        )
    );
}
impl Default for ShouldNotBeCopy {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

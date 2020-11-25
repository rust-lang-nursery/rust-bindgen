#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct NoHash {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_NoHash() {
    assert_eq!(
        ::std::mem::size_of::<NoHash>(),
        1usize,
        concat!("Size of: ", stringify!(NoHash))
    );
    assert_eq!(
        ::std::mem::align_of::<NoHash>(),
        1usize,
        concat!("Alignment of ", stringify!(NoHash))
    );
}
struct Box_NoHash {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_NoHash {}
impl Drop for Box_NoHash {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct WhitelistMe {
    pub a: NoHash,
}
#[test]
fn bindgen_test_layout_WhitelistMe() {
    assert_eq!(
        ::std::mem::size_of::<WhitelistMe>(),
        1usize,
        concat!("Size of: ", stringify!(WhitelistMe))
    );
    assert_eq!(
        ::std::mem::align_of::<WhitelistMe>(),
        1usize,
        concat!("Alignment of ", stringify!(WhitelistMe))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WhitelistMe>())).a as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WhitelistMe),
            "::",
            stringify!(a)
        )
    );
}
struct Box_WhitelistMe {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_WhitelistMe {}
impl Drop for Box_WhitelistMe {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
}

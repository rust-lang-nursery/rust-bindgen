#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct header {
    pub _bindgen_opaque_blob: [u8; 16usize],
}
#[test]
fn bindgen_test_layout_header() {
    assert_eq!(
        ::std::mem::size_of::<header>(),
        16usize,
        concat!("Size of: ", stringify!(header))
    );
}
impl Default for header {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_header {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_header {}
impl Drop for Box_header {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(16usize, 16usize)
                    .unwrap(),
            );
        }
    }
}

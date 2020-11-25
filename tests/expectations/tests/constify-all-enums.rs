#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const foo_THIS: foo = 0;
pub const foo_SHOULD_BE: foo = 1;
pub const foo_A_CONSTANT: foo = 2;
pub type foo = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bar {
    pub this_should_work: foo,
}
#[test]
fn bindgen_test_layout_bar() {
    assert_eq!(
        ::std::mem::size_of::<bar>(),
        4usize,
        concat!("Size of: ", stringify!(bar))
    );
    assert_eq!(
        ::std::mem::align_of::<bar>(),
        4usize,
        concat!("Alignment of ", stringify!(bar))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<bar>())).this_should_work as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bar),
            "::",
            stringify!(this_should_work)
        )
    );
}
impl Default for bar {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_bar {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_bar {}
impl Drop for Box_bar {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}

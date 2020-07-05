#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Base {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Derived {
    pub b: bool,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Usage {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_ZN5Usage13static_memberE"]
    pub static mut Usage_static_member: [u32; 2usize];
}
#[test]
fn bindgen_test_layout_Usage() {
    assert_eq!(
        ::std::mem::size_of::<Usage>(),
        1usize,
        concat!("Size of: ", stringify!(Usage))
    );
    assert_eq!(
        ::std::mem::align_of::<Usage>(),
        1usize,
        concat!("Alignment of ", stringify!(Usage))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN5UsageC1Ev"]
    pub fn Usage_Usage(this: *mut Usage);
}
impl Usage {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        Usage_Usage(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
}

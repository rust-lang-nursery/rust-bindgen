#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Nice {
    pub pointer: Nice_Function,
    pub large_array: [::std::os::raw::c_int; 34usize],
}
pub type Nice_Function =
    ::std::option::Option<unsafe extern "C" fn(data: ::std::os::raw::c_int)>;
#[test]
fn bindgen_test_layout_Nice() {
    assert_eq!(
        ::std::mem::size_of::<Nice>(),
        144usize,
        concat!("Size of: ", stringify!(Nice))
    );
    assert_eq!(
        ::std::mem::align_of::<Nice>(),
        8usize,
        concat!("Alignment of ", stringify!(Nice))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Nice>())).pointer as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Nice),
            "::",
            stringify!(pointer)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Nice>())).large_array as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Nice),
            "::",
            stringify!(large_array)
        )
    );
}
impl Default for Nice {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Nice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(
            f,
            "Nice {{ pointer: {:?}, large_array: [{}] }}",
            self.pointer,
            self.large_array
                .iter()
                .enumerate()
                .map(|(i, v)| format!(
                    "{}{:?}",
                    if i > 0 { ", " } else { "" },
                    v
                ))
                .collect::<String>()
        )
    }
}
struct Box_Nice {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Nice {}
impl Drop for Box_Nice {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(144usize, 8usize)
                    .unwrap(),
            );
        }
    }
}

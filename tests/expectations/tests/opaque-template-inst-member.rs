#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OpaqueTemplate {
    pub _address: u8,
}
/// This should not end up deriving Debug/Hash because its `mBlah` field cannot derive
/// Debug/Hash because the instantiation's definition cannot derive Debug/Hash.
#[repr(C)]
pub struct ContainsOpaqueTemplate {
    pub mBlah: [u32; 101usize],
    pub mBaz: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ContainsOpaqueTemplate() {
    assert_eq!(
        ::std::mem::size_of::<ContainsOpaqueTemplate>(),
        408usize,
        concat!("Size of: ", stringify!(ContainsOpaqueTemplate))
    );
    assert_eq!(
        ::std::mem::align_of::<ContainsOpaqueTemplate>(),
        4usize,
        concat!("Alignment of ", stringify!(ContainsOpaqueTemplate))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ContainsOpaqueTemplate>())).mBlah as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ContainsOpaqueTemplate),
            "::",
            stringify!(mBlah)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ContainsOpaqueTemplate>())).mBaz as *const _
                as usize
        },
        404usize,
        concat!(
            "Offset of field: ",
            stringify!(ContainsOpaqueTemplate),
            "::",
            stringify!(mBaz)
        )
    );
}
impl Default for ContainsOpaqueTemplate {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ContainsOpaqueTemplate {
    fn eq(&self, other: &ContainsOpaqueTemplate) -> bool {
        &self.mBlah[..] == &other.mBlah[..] && self.mBaz == other.mBaz
    }
}
struct Box_ContainsOpaqueTemplate {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_ContainsOpaqueTemplate {}
impl Drop for Box_ContainsOpaqueTemplate {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(408usize, 4usize)
                    .unwrap(),
            );
        }
    }
}
/// This should not end up deriving Debug/Hash either, for similar reasons, although
/// we're exercising base member edges now.
#[repr(C)]
pub struct InheritsOpaqueTemplate {
    pub _base: [u8; 401usize],
    pub wow: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_InheritsOpaqueTemplate() {
    assert_eq!(
        ::std::mem::size_of::<InheritsOpaqueTemplate>(),
        416usize,
        concat!("Size of: ", stringify!(InheritsOpaqueTemplate))
    );
    assert_eq!(
        ::std::mem::align_of::<InheritsOpaqueTemplate>(),
        8usize,
        concat!("Alignment of ", stringify!(InheritsOpaqueTemplate))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<InheritsOpaqueTemplate>())).wow as *const _
                as usize
        },
        408usize,
        concat!(
            "Offset of field: ",
            stringify!(InheritsOpaqueTemplate),
            "::",
            stringify!(wow)
        )
    );
}
impl Default for InheritsOpaqueTemplate {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for InheritsOpaqueTemplate {
    fn eq(&self, other: &InheritsOpaqueTemplate) -> bool {
        &self._base[..] == &other._base[..] && self.wow == other.wow
    }
}
struct Box_InheritsOpaqueTemplate {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_InheritsOpaqueTemplate {}
impl Drop for Box_InheritsOpaqueTemplate {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(416usize, 8usize)
                    .unwrap(),
            );
        }
    }
}

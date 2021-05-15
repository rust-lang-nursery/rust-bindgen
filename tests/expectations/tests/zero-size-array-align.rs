#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct dm_deps {
    pub count: ::std::os::raw::c_uint,
    pub filler: ::std::os::raw::c_uint,
    pub device: __IncompleteArrayField<::std::os::raw::c_ulonglong>,
}
#[test]
fn bindgen_test_layout_dm_deps() {
    assert_eq!(
        ::std::mem::size_of::<dm_deps>(),
        8usize,
        concat!("Size of: ", stringify!(dm_deps))
    );
    assert_eq!(
        ::std::mem::align_of::<dm_deps>(),
        8usize,
        concat!("Alignment of ", stringify!(dm_deps))
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<dm_deps>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], dm_deps>(buffer)
            };
            let struct_ptr = &struct_instance as *const dm_deps;
            let field_ptr = std::ptr::addr_of!(struct_instance.count);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(dm_deps),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<dm_deps>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], dm_deps>(buffer)
            };
            let struct_ptr = &struct_instance as *const dm_deps;
            let field_ptr = std::ptr::addr_of!(struct_instance.filler);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(dm_deps),
            "::",
            stringify!(filler)
        )
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<dm_deps>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], dm_deps>(buffer)
            };
            let struct_ptr = &struct_instance as *const dm_deps;
            let field_ptr = std::ptr::addr_of!(struct_instance.device);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(dm_deps),
            "::",
            stringify!(device)
        )
    );
}

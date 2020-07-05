#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BaseUsesT<T> {
    pub usage: *mut T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for BaseUsesT<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CrtpIgnoresU {
    pub _base: BaseUsesT<CrtpIgnoresU>,
    pub y: ::std::os::raw::c_int,
}
impl Default for CrtpIgnoresU {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

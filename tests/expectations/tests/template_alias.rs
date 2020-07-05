#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type JS_detail_Wrapped<T> = T;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct JS_Rooted<T> {
    pub ptr: JS_detail_Wrapped<T>,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for JS_Rooted<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

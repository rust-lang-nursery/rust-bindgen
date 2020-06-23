#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DoesNotUseU<T, V> {
    pub t: T,
    pub v: V,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<V>>,
}
impl<T, V> Default for DoesNotUseU<T, V> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type Alias = DoesNotUseU<::std::os::raw::c_int, ::std::os::raw::c_char>;

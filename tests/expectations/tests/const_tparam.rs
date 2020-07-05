#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct C<T> {
    pub foo: *const T,
    pub bar: *const T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for C<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UsesTemplateParameter<T> {
    pub t: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct UsesTemplateParameter_DoesNotUseTemplateParameters {
    pub x: ::std::os::raw::c_int,
}
impl<T> Default for UsesTemplateParameter<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

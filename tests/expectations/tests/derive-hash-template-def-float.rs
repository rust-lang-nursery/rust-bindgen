#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// Template definition containing a float, which cannot derive Hash/Eq/Ord but can derive PartialEq/PartialOrd.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct foo<T> {
    pub data: T,
    pub b: f32,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for foo<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

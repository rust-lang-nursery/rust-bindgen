#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Wrapper_Wrapped<T> {
    pub t: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for Wrapper_Wrapped<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type Wrapper_Type<T> = Wrapper_Wrapped<T>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rooted<T> {
    pub ptr: Rooted_MaybeWrapped<T>,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
/// <div rustbindgen replaces="Rooted_MaybeWrapped"></div>
pub type Rooted_MaybeWrapped<T> = T;
impl<T> Default for Rooted<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

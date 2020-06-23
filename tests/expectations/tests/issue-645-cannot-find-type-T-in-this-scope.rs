#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[derive(Clone, Copy, Debug)]
pub struct RefPtr<T>(T);

#[repr(C)]
pub struct HasRefPtr<T> {
    pub refptr_member: RefPtr<HasRefPtr_TypedefOfT<T>>,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
pub type HasRefPtr_TypedefOfT<T> = T;
impl<T> Default for HasRefPtr<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

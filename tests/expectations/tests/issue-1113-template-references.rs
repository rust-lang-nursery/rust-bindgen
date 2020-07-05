#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Entry<K, V> {
    pub _base: K,
    pub mData: V,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<K>>,
    pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<V>>,
}
impl<K, V> Default for Entry<K, V> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nsBaseHashtable {
    pub _address: u8,
}
pub type nsBaseHashtable_EntryType<K, V> = Entry<K, V>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsBaseHashtable_EntryPtr<K, V> {
    pub mEntry: *mut nsBaseHashtable_EntryType<K, V>,
    pub mExistingEntry: bool,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<K>>,
    pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<V>>,
}
impl<K, V> Default for nsBaseHashtable_EntryPtr<K, V> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

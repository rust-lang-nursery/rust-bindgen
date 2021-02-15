#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct Blacklisted<T> {
    t: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}

///```text
/// This would derive(Hash, Eq, PartialEq) if it didn't contain a blacklisted type,
/// causing us to conservatively avoid deriving hash/Eq/PartialEq for it.
///```
#[repr(C)]
pub struct WhitelistedOne {
    pub a: Blacklisted<::std::os::raw::c_int>,
}
#[test]
fn bindgen_test_layout_WhitelistedOne() {
    assert_eq!(
        ::std::mem::size_of::<WhitelistedOne>(),
        4usize,
        concat!("Size of: ", stringify!(WhitelistedOne))
    );
    assert_eq!(
        ::std::mem::align_of::<WhitelistedOne>(),
        4usize,
        concat!("Alignment of ", stringify!(WhitelistedOne))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WhitelistedOne>())).a as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WhitelistedOne),
            "::",
            stringify!(a)
        )
    );
}
impl Default for WhitelistedOne {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
///```text
/// This can't derive(Hash/Eq) even if it didn't contain a blacklisted type.
///```
#[repr(C)]
pub struct WhitelistedTwo {
    pub b: Blacklisted<f32>,
}
#[test]
fn bindgen_test_layout_WhitelistedTwo() {
    assert_eq!(
        ::std::mem::size_of::<WhitelistedTwo>(),
        4usize,
        concat!("Size of: ", stringify!(WhitelistedTwo))
    );
    assert_eq!(
        ::std::mem::align_of::<WhitelistedTwo>(),
        4usize,
        concat!("Alignment of ", stringify!(WhitelistedTwo))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WhitelistedTwo>())).b as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WhitelistedTwo),
            "::",
            stringify!(b)
        )
    );
}
impl Default for WhitelistedTwo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

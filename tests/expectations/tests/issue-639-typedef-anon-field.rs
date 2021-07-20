#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub bar: Foo_Bar,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo_Bar {
    pub abc: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Foo_Bar() {
    assert_eq!(
        ::std::mem::size_of::<Foo_Bar>(),
        4usize,
        concat!("Size of: ", stringify!(Foo_Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo_Bar>(),
        4usize,
        concat!("Alignment of ", stringify!(Foo_Bar))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Foo_Bar>() };
            let struct_ptr = &struct_instance as *const Foo_Bar;
            let field_ptr = std::ptr::addr_of!(struct_instance.abc);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Foo_Bar),
            "::",
            stringify!(abc)
        )
    );
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(
        ::std::mem::size_of::<Foo>(),
        4usize,
        concat!("Size of: ", stringify!(Foo))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo>(),
        4usize,
        concat!("Alignment of ", stringify!(Foo))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Foo>() };
            let struct_ptr = &struct_instance as *const Foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.bar);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(Foo), "::", stringify!(bar))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Baz {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Baz_Bar {
    pub abc: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Baz_Bar() {
    assert_eq!(
        ::std::mem::size_of::<Baz_Bar>(),
        4usize,
        concat!("Size of: ", stringify!(Baz_Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Baz_Bar>(),
        4usize,
        concat!("Alignment of ", stringify!(Baz_Bar))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Baz_Bar>() };
            let struct_ptr = &struct_instance as *const Baz_Bar;
            let field_ptr = std::ptr::addr_of!(struct_instance.abc);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Baz_Bar),
            "::",
            stringify!(abc)
        )
    );
}
#[test]
fn bindgen_test_layout_Baz() {
    assert_eq!(
        ::std::mem::size_of::<Baz>(),
        1usize,
        concat!("Size of: ", stringify!(Baz))
    );
    assert_eq!(
        ::std::mem::align_of::<Baz>(),
        1usize,
        concat!("Alignment of ", stringify!(Baz))
    );
}

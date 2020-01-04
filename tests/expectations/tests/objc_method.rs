/* automatically generated by rust-bindgen */

#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(target_os = "macos")]

#[macro_use]
extern crate objc;
#[allow(non_camel_case_types)]
pub type id = *mut objc::runtime::Object;
pub trait Foo {
    unsafe fn method(self);
    unsafe fn methodWithInt_(self, foo: ::std::os::raw::c_int);
    unsafe fn methodWithFoo_(self, foo: id);
    unsafe fn methodReturningInt(self) -> ::std::os::raw::c_int;
    unsafe fn methodReturningFoo(self) -> *mut id;
    unsafe fn methodWithArg1_andArg2_andArg3_(
        self,
        intvalue: ::std::os::raw::c_int,
        ptr: *mut ::std::os::raw::c_char,
        floatvalue: f32,
    );
    unsafe fn methodWithAndWithoutKeywords_arg2Name__arg4Name_(
        self,
        arg1: ::std::os::raw::c_int,
        arg2: f32,
        arg3: f32,
        arg4: ::std::os::raw::c_int,
    ) -> instancetype;
}
impl Foo for id {
    unsafe fn method(self) {
        msg_send!(self, method)
    }
    unsafe fn methodWithInt_(self, foo: ::std::os::raw::c_int) {
        msg_send!(self, methodWithInt: foo)
    }
    unsafe fn methodWithFoo_(self, foo: id) {
        msg_send!(self, methodWithFoo: foo)
    }
    unsafe fn methodReturningInt(self) -> ::std::os::raw::c_int {
        msg_send!(self, methodReturningInt)
    }
    unsafe fn methodReturningFoo(self) -> *mut id {
        msg_send!(self, methodReturningFoo)
    }
    unsafe fn methodWithArg1_andArg2_andArg3_(
        self,
        intvalue: ::std::os::raw::c_int,
        ptr: *mut ::std::os::raw::c_char,
        floatvalue: f32,
    ) {
        msg_send ! ( self , methodWithArg1 : intvalue andArg2 : ptr andArg3 : floatvalue )
    }
    unsafe fn methodWithAndWithoutKeywords_arg2Name__arg4Name_(
        self,
        arg1: ::std::os::raw::c_int,
        arg2: f32,
        arg3: f32,
        arg4: ::std::os::raw::c_int,
    ) -> instancetype {
        msg_send ! ( self , methodWithAndWithoutKeywords : arg1 arg2Name : arg2 arg3 : arg3 arg4Name : arg4 )
    }
}
pub type instancetype = id;

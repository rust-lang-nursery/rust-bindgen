#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align> {
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                self.storage.as_ref().len()
        );
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                self.storage.as_ref().len()
        );
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union U4 {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_U4() {
    assert_eq!(
        ::std::mem::size_of::<U4>(),
        4usize,
        concat!("Size of: ", stringify!(U4))
    );
    assert_eq!(
        ::std::mem::align_of::<U4>(),
        4usize,
        concat!("Alignment of ", stringify!(U4))
    );
}
impl Default for U4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl U4 {
    #[inline]
    pub fn derp(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32)
        }
    }
    #[inline]
    pub fn set_derp(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        derp: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<
            [u8; 1usize],
            u8,
        > = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let derp: u32 = unsafe { ::std::mem::transmute(derp) };
            derp as u64
        });
        __bindgen_bitfield_unit
    }
}
struct Box_U4 {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_U4 {}
impl Drop for Box_U4 {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union B {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u32>,
    _bindgen_union_align: u32,
}
#[test]
fn bindgen_test_layout_B() {
    assert_eq!(
        ::std::mem::size_of::<B>(),
        4usize,
        concat!("Size of: ", stringify!(B))
    );
    assert_eq!(
        ::std::mem::align_of::<B>(),
        4usize,
        concat!("Alignment of ", stringify!(B))
    );
}
impl Default for B {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl B {
    #[inline]
    pub fn foo(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 31u8) as u32)
        }
    }
    #[inline]
    pub fn set_foo(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 31u8, val as u64)
        }
    }
    #[inline]
    pub fn bar(&self) -> ::std::os::raw::c_uchar {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(31usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_bar(&mut self, val: ::std::os::raw::c_uchar) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(31usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        foo: ::std::os::raw::c_uint,
        bar: ::std::os::raw::c_uchar,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u32> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<
            [u8; 4usize],
            u32,
        > = Default::default();
        __bindgen_bitfield_unit.set(0usize, 31u8, {
            let foo: u32 = unsafe { ::std::mem::transmute(foo) };
            foo as u64
        });
        __bindgen_bitfield_unit.set(31usize, 1u8, {
            let bar: u8 = unsafe { ::std::mem::transmute(bar) };
            bar as u64
        });
        __bindgen_bitfield_unit
    }
}
struct Box_B {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_B {}
impl Drop for Box_B {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}

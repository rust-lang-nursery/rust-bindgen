#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const RTE_CACHE_LINE_SIZE: u32 = 64;
pub const RTE_MEMPOOL_OPS_NAMESIZE: u32 = 32;
pub const RTE_MEMPOOL_MAX_OPS_IDX: u32 = 16;
pub const RTE_HEAP_NUM_FREELISTS: u32 = 13;
pub type size_t = ::std::os::raw::c_longlong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rte_mempool {
    _unused: [u8; 0],
}
/// Prototype for implementation specific data provisioning function.
///
/// The function should provide the implementation specific memory for
/// for use by the other mempool ops functions in a given mempool ops struct.
/// E.g. the default ops provides an instance of the rte_ring for this purpose.
/// it will most likely point to a different type of data structure, and
/// will be transparent to the application programmer.
/// This function should set mp->pool_data.
pub type rte_mempool_alloc_t = ::std::option::Option<
    unsafe extern "C" fn(mp: *mut rte_mempool) -> ::std::os::raw::c_int,
>;
/// Free the opaque private data pointed to by mp->pool_data pointer.
pub type rte_mempool_free_t =
    ::std::option::Option<unsafe extern "C" fn(mp: *mut rte_mempool)>;
/// Enqueue an object into the external pool.
pub type rte_mempool_enqueue_t = ::std::option::Option<
    unsafe extern "C" fn(
        mp: *mut rte_mempool,
        obj_table: *const *mut ::std::os::raw::c_void,
        n: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int,
>;
/// Dequeue an object from the external pool.
pub type rte_mempool_dequeue_t = ::std::option::Option<
    unsafe extern "C" fn(
        mp: *mut rte_mempool,
        obj_table: *mut *mut ::std::os::raw::c_void,
        n: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int,
>;
/// Return the number of available objects in the external pool.
pub type rte_mempool_get_count = ::std::option::Option<
    unsafe extern "C" fn(mp: *const rte_mempool) -> ::std::os::raw::c_uint,
>;
/// Structure defining mempool operations structure
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct rte_mempool_ops {
    ///< Name of mempool ops struct.
    pub name: [::std::os::raw::c_char; 32usize],
    ///< Allocate private data.
    pub alloc: rte_mempool_alloc_t,
    ///< Free the external pool.
    pub free: rte_mempool_free_t,
    ///< Enqueue an object.
    pub enqueue: rte_mempool_enqueue_t,
    ///< Dequeue an object.
    pub dequeue: rte_mempool_dequeue_t,
    ///< Get qty of available objs.
    pub get_count: rte_mempool_get_count,
}
#[test]
fn bindgen_test_layout_rte_mempool_ops() {
    assert_eq!(
        ::std::mem::size_of::<rte_mempool_ops>(),
        128usize,
        concat!("Size of: ", stringify!(rte_mempool_ops))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mempool_ops>(),
        64usize,
        concat!("Alignment of ", stringify!(rte_mempool_ops))
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<rte_mempool_ops>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], rte_mempool_ops>(
                    buffer,
                )
            };
            let struct_ptr = &struct_instance as *const rte_mempool_ops;
            let field_ptr = std::ptr::addr_of!(struct_instance.name);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mempool_ops),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<rte_mempool_ops>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], rte_mempool_ops>(
                    buffer,
                )
            };
            let struct_ptr = &struct_instance as *const rte_mempool_ops;
            let field_ptr = std::ptr::addr_of!(struct_instance.alloc);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mempool_ops),
            "::",
            stringify!(alloc)
        )
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<rte_mempool_ops>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], rte_mempool_ops>(
                    buffer,
                )
            };
            let struct_ptr = &struct_instance as *const rte_mempool_ops;
            let field_ptr = std::ptr::addr_of!(struct_instance.free);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mempool_ops),
            "::",
            stringify!(free)
        )
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<rte_mempool_ops>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], rte_mempool_ops>(
                    buffer,
                )
            };
            let struct_ptr = &struct_instance as *const rte_mempool_ops;
            let field_ptr = std::ptr::addr_of!(struct_instance.enqueue);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mempool_ops),
            "::",
            stringify!(enqueue)
        )
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<rte_mempool_ops>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], rte_mempool_ops>(
                    buffer,
                )
            };
            let struct_ptr = &struct_instance as *const rte_mempool_ops;
            let field_ptr = std::ptr::addr_of!(struct_instance.dequeue);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mempool_ops),
            "::",
            stringify!(dequeue)
        )
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<rte_mempool_ops>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], rte_mempool_ops>(
                    buffer,
                )
            };
            let struct_ptr = &struct_instance as *const rte_mempool_ops;
            let field_ptr = std::ptr::addr_of!(struct_instance.get_count);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mempool_ops),
            "::",
            stringify!(get_count)
        )
    );
}
impl Default for rte_mempool_ops {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::cmp::PartialEq for rte_mempool_ops {
    fn eq(&self, other: &rte_mempool_ops) -> bool {
        self.name == other.name &&
            self.alloc == other.alloc &&
            self.free == other.free &&
            self.enqueue == other.enqueue &&
            self.dequeue == other.dequeue &&
            self.get_count == other.get_count
    }
}
/// The rte_spinlock_t type.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_spinlock_t {
    ///< lock status 0 = unlocked, 1 = locked
    pub locked: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_rte_spinlock_t() {
    assert_eq!(
        ::std::mem::size_of::<rte_spinlock_t>(),
        4usize,
        concat!("Size of: ", stringify!(rte_spinlock_t))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_spinlock_t>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_spinlock_t))
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<rte_spinlock_t>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], rte_spinlock_t>(buffer)
            };
            let struct_ptr = &struct_instance as *const rte_spinlock_t;
            let field_ptr = std::ptr::addr_of!(struct_instance.locked);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_spinlock_t),
            "::",
            stringify!(locked)
        )
    );
}
/// Structure storing the table of registered ops structs, each of which contain
/// the function pointers for the mempool ops functions.
/// Each process has its own storage for this ops struct array so that
/// the mempools can be shared across primary and secondary processes.
/// The indices used to access the array are valid across processes, whereas
/// any function pointers stored directly in the mempool struct would not be.
/// This results in us simply having "ops_index" in the mempool struct.
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct rte_mempool_ops_table {
    ///< Spinlock for add/delete.
    pub sl: rte_spinlock_t,
    ///< Number of used ops structs in the table.
    pub num_ops: u32,
    pub __bindgen_padding_0: [u64; 7usize],
    /// Storage for all possible ops structs.
    pub ops: [rte_mempool_ops; 16usize],
}
#[test]
fn bindgen_test_layout_rte_mempool_ops_table() {
    assert_eq!(
        ::std::mem::size_of::<rte_mempool_ops_table>(),
        2112usize,
        concat!("Size of: ", stringify!(rte_mempool_ops_table))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mempool_ops_table>(),
        64usize,
        concat!("Alignment of ", stringify!(rte_mempool_ops_table))
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize =
                std::mem::size_of::<rte_mempool_ops_table>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], rte_mempool_ops_table>(
                    buffer,
                )
            };
            let struct_ptr = &struct_instance as *const rte_mempool_ops_table;
            let field_ptr = std::ptr::addr_of!(struct_instance.sl);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mempool_ops_table),
            "::",
            stringify!(sl)
        )
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize =
                std::mem::size_of::<rte_mempool_ops_table>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], rte_mempool_ops_table>(
                    buffer,
                )
            };
            let struct_ptr = &struct_instance as *const rte_mempool_ops_table;
            let field_ptr = std::ptr::addr_of!(struct_instance.num_ops);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mempool_ops_table),
            "::",
            stringify!(num_ops)
        )
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize =
                std::mem::size_of::<rte_mempool_ops_table>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], rte_mempool_ops_table>(
                    buffer,
                )
            };
            let struct_ptr = &struct_instance as *const rte_mempool_ops_table;
            let field_ptr = std::ptr::addr_of!(struct_instance.ops);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_mempool_ops_table),
            "::",
            stringify!(ops)
        )
    );
}
impl Default for rte_mempool_ops_table {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// Structure to hold malloc heap
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct malloc_heap {
    pub lock: rte_spinlock_t,
    pub free_head: [malloc_heap__bindgen_ty_1; 13usize],
    pub alloc_count: ::std::os::raw::c_uint,
    pub total_size: size_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct malloc_heap__bindgen_ty_1 {
    pub lh_first: *mut malloc_elem,
}
#[test]
fn bindgen_test_layout_malloc_heap__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<malloc_heap__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(malloc_heap__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<malloc_heap__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(malloc_heap__bindgen_ty_1))
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize =
                std::mem::size_of::<malloc_heap__bindgen_ty_1>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<
                    [u8; STRUCT_SIZE],
                    malloc_heap__bindgen_ty_1,
                >(buffer)
            };
            let struct_ptr =
                &struct_instance as *const malloc_heap__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.lh_first);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(malloc_heap__bindgen_ty_1),
            "::",
            stringify!(lh_first)
        )
    );
}
impl Default for malloc_heap__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_malloc_heap() {
    assert_eq!(
        ::std::mem::size_of::<malloc_heap>(),
        128usize,
        concat!("Size of: ", stringify!(malloc_heap))
    );
    assert_eq!(
        ::std::mem::align_of::<malloc_heap>(),
        64usize,
        concat!("Alignment of ", stringify!(malloc_heap))
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<malloc_heap>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], malloc_heap>(buffer)
            };
            let struct_ptr = &struct_instance as *const malloc_heap;
            let field_ptr = std::ptr::addr_of!(struct_instance.lock);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(malloc_heap),
            "::",
            stringify!(lock)
        )
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<malloc_heap>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], malloc_heap>(buffer)
            };
            let struct_ptr = &struct_instance as *const malloc_heap;
            let field_ptr = std::ptr::addr_of!(struct_instance.free_head);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(malloc_heap),
            "::",
            stringify!(free_head)
        )
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<malloc_heap>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], malloc_heap>(buffer)
            };
            let struct_ptr = &struct_instance as *const malloc_heap;
            let field_ptr = std::ptr::addr_of!(struct_instance.alloc_count);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(malloc_heap),
            "::",
            stringify!(alloc_count)
        )
    );
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<malloc_heap>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], malloc_heap>(buffer)
            };
            let struct_ptr = &struct_instance as *const malloc_heap;
            let field_ptr = std::ptr::addr_of!(struct_instance.total_size);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(malloc_heap),
            "::",
            stringify!(total_size)
        )
    );
}
impl Default for malloc_heap {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::cmp::PartialEq for malloc_heap {
    fn eq(&self, other: &malloc_heap) -> bool {
        self.lock == other.lock &&
            self.free_head == other.free_head &&
            self.alloc_count == other.alloc_count &&
            self.total_size == other.total_size
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct malloc_elem {
    pub _address: u8,
}

#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type mbedtls_mpi_uint = ::std::os::raw::c_uint;
/// \brief          MPI structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mbedtls_mpi {
    ///<  integer sign
    pub s: ::std::os::raw::c_int,
    ///<  total # of limbs
    pub n: ::std::os::raw::c_ulong,
    ///<  pointer to limbs
    pub p: *mut mbedtls_mpi_uint,
}
#[test]
fn bindgen_test_layout_mbedtls_mpi() {
    assert_eq!(
        ::std::mem::size_of::<mbedtls_mpi>(),
        24usize,
        concat!("Size of: ", stringify!(mbedtls_mpi))
    );
    assert_eq!(
        ::std::mem::align_of::<mbedtls_mpi>(),
        8usize,
        concat!("Alignment of ", stringify!(mbedtls_mpi))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<mbedtls_mpi>())).s as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mbedtls_mpi),
            "::",
            stringify!(s)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<mbedtls_mpi>())).n as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(mbedtls_mpi),
            "::",
            stringify!(n)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<mbedtls_mpi>())).p as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(mbedtls_mpi),
            "::",
            stringify!(p)
        )
    );
}
impl Default for mbedtls_mpi {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_mbedtls_mpi {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_mbedtls_mpi {}
impl Drop for Box_mbedtls_mpi {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(24usize, 8usize).unwrap(),
            );
        }
    }
}

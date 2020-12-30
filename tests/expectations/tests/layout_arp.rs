#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const ETHER_ADDR_LEN: u32 = 6;
pub const ARP_HRD_ETHER: u32 = 1;
pub const ARP_OP_REQUEST: u32 = 1;
pub const ARP_OP_REPLY: u32 = 2;
pub const ARP_OP_REVREQUEST: u32 = 3;
pub const ARP_OP_REVREPLY: u32 = 4;
pub const ARP_OP_INVREQUEST: u32 = 8;
pub const ARP_OP_INVREPLY: u32 = 9;
///```text
/// Ethernet address:
/// A universally administered address is uniquely assigned to a device by its
/// manufacturer. The first three octets (in transmission order) contain the
/// Organizationally Unique Identifier (OUI). The following three (MAC-48 and
/// EUI-48) octets are assigned by that organization with the only constraint
/// of uniqueness.
/// A locally administered address is assigned to a device by a network
/// administrator and does not contain OUIs.
/// See http://standards.ieee.org/regauth/groupmac/tutorial.html
///```
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ether_addr {
    ///```text
    ///< Addr bytes in tx order
    ///```
    pub addr_bytes: [u8; 6usize],
}
#[test]
fn bindgen_test_layout_ether_addr() {
    assert_eq!(
        ::std::mem::size_of::<ether_addr>(),
        6usize,
        concat!("Size of: ", stringify!(ether_addr))
    );
    assert_eq!(
        ::std::mem::align_of::<ether_addr>(),
        1usize,
        concat!("Alignment of ", stringify!(ether_addr))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ether_addr>())).addr_bytes as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ether_addr),
            "::",
            stringify!(addr_bytes)
        )
    );
}
///```text
/// ARP header IPv4 payload.
///```
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct arp_ipv4 {
    ///```text
    ///< sender hardware address
    ///```
    pub arp_sha: ether_addr,
    ///```text
    ///< sender IP address
    ///```
    pub arp_sip: u32,
    ///```text
    ///< target hardware address
    ///```
    pub arp_tha: ether_addr,
    ///```text
    ///< target IP address
    ///```
    pub arp_tip: u32,
}
#[test]
fn bindgen_test_layout_arp_ipv4() {
    assert_eq!(
        ::std::mem::size_of::<arp_ipv4>(),
        20usize,
        concat!("Size of: ", stringify!(arp_ipv4))
    );
    assert_eq!(
        ::std::mem::align_of::<arp_ipv4>(),
        1usize,
        concat!("Alignment of ", stringify!(arp_ipv4))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<arp_ipv4>())).arp_sha as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_ipv4),
            "::",
            stringify!(arp_sha)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<arp_ipv4>())).arp_sip as *const _ as usize
        },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_ipv4),
            "::",
            stringify!(arp_sip)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<arp_ipv4>())).arp_tha as *const _ as usize
        },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_ipv4),
            "::",
            stringify!(arp_tha)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<arp_ipv4>())).arp_tip as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_ipv4),
            "::",
            stringify!(arp_tip)
        )
    );
}
///```text
/// ARP header.
///```
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct arp_hdr {
    pub arp_hrd: u16,
    pub arp_pro: u16,
    pub arp_hln: u8,
    pub arp_pln: u8,
    pub arp_op: u16,
    pub arp_data: arp_ipv4,
}
#[test]
fn bindgen_test_layout_arp_hdr() {
    assert_eq!(
        ::std::mem::size_of::<arp_hdr>(),
        28usize,
        concat!("Size of: ", stringify!(arp_hdr))
    );
    assert_eq!(
        ::std::mem::align_of::<arp_hdr>(),
        1usize,
        concat!("Alignment of ", stringify!(arp_hdr))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<arp_hdr>())).arp_hrd as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_hdr),
            "::",
            stringify!(arp_hrd)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<arp_hdr>())).arp_pro as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_hdr),
            "::",
            stringify!(arp_pro)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<arp_hdr>())).arp_hln as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_hdr),
            "::",
            stringify!(arp_hln)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<arp_hdr>())).arp_pln as *const _ as usize
        },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_hdr),
            "::",
            stringify!(arp_pln)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<arp_hdr>())).arp_op as *const _ as usize
        },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_hdr),
            "::",
            stringify!(arp_op)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<arp_hdr>())).arp_data as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(arp_hdr),
            "::",
            stringify!(arp_data)
        )
    );
}

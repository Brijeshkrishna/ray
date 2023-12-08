#![allow(unused)]

use crate::dev_info::copy_interface;

use libc::{
    __errno_location, ifreq, in_addr, ioctl, sockaddr, sockaddr_in, socket, strerror, AF_INET,
};

use std::{ffi::CStr, net::Ipv4Addr, str::FromStr};

macro_rules! print_error {
    ($prefix:expr $(, $subfix:expr)?) => {
        let rstr = CStr::from_ptr(strerror(*__errno_location())).to_str().unwrap();
        print!("{}: {}", $prefix, rstr);
        $(print!(" {}", $subfix);)?
    };
}

pub struct InetModify {
    pub sock: i32,
    pub ifr: libc::ifreq,
}

macro_rules! to_ptr {
    ($x:expr) => {
        //  $x as *mut _ as *mut std::ffi::c_void
        &($x) as *const _ as *mut std::ffi::c_void
    };
}

macro_rules! sizeof {
    ($ty:ty) => {
        std::mem::size_of::<$ty>()
    };
}

macro_rules! copy_to_sockaddr {
    ($new_ip:expr,$req:expr) => {
        unsafe {
            std::ptr::copy(
                to_ptr!(ip_to_sockaddr_in($new_ip)),
                to_ptr!($req),
                sizeof!(sockaddr_in),
            );
        }
    };
}

pub fn copy_ip(ip: &Ipv4Addr, ifr: &mut ifreq) {
    let mut addr = [0i8; 14];

    let octets = ip.octets();

    addr[2] = octets[0] as i8;
    addr[3] = octets[1] as i8;
    addr[4] = octets[2] as i8;
    addr[5] = octets[3] as i8;

    ifr.ifr_ifru.ifru_dstaddr.sa_family = AF_INET as u16;
    ifr.ifr_ifru.ifru_dstaddr.sa_data = addr;
}

impl InetModify {
    pub fn new(interface: &str) -> Option<Self> {
        let mut inet = unsafe {
            InetModify {
                sock: socket(libc::AF_INET, libc::SOCK_DGRAM, 0),
                ifr: std::mem::zeroed(),
            }
        };

        copy_interface(&mut inet.ifr, interface);
        if unsafe { ioctl(inet.sock, libc::SIOCGIFFLAGS, &mut inet.ifr) } == 0 {
            return Some(inet);
        }
        None
    }

    fn call(&mut self, req: u64, req_name: &str) {
        unsafe {
            if ioctl(self.sock, req, &mut self.ifr) != 0 {
                print_error!(req_name, "(Suggestion: try with sudo)\n");
                std::process::exit(-1)
            }
        }
    }

    pub fn interface_up(&mut self) {
        self.call(libc::SIOCGIFFLAGS, "SIOCGIFFLAGS");

        unsafe {
            self.ifr.ifr_ifru.ifru_flags = self.ifr.ifr_ifru.ifru_flags | libc::IFF_UP as i16
        };

        self.call(libc::SIOCSIFFLAGS, "SIOCSIFFLAGS");
    }

    pub fn interface_down(&mut self) {
        self.call(libc::SIOCGIFFLAGS, "SIOCGIFFLAGS");
        unsafe {
            self.ifr.ifr_ifru.ifru_flags =
                (self.ifr.ifr_ifru.ifru_flags as i32 & (!libc::IFF_UP)) as i16
        };
        self.call(libc::SIOCSIFFLAGS, "SIOCSIFFLAGS");
    }
    pub fn rename_interface(&mut self, new_interface: &str) {
        copy_interface(&mut self.ifr, new_interface);
        // copy_new_interface(new_interface, &mut self.ifr);
        self.interface_down();
        self.call(libc::SIOCSIFNAME, "SIOCSIFNAME");
        self.interface_up();
    }

    pub fn change_mac(&mut self, new_mac: &str) {
        copy_mac(new_mac, &mut self.ifr);
        self.call(libc::SIOCSIFHWADDR, "SIOCSIFHWADDR");
    }

    pub fn add_ip(&mut self, new_ip: &str) {
        copy_to_sockaddr!(new_ip, self.ifr.ifr_ifru.ifru_addr);
        self.call(libc::SIOCSIFADDR, "SIOCSIFADDR");
    }

    pub fn change_dest_ip(&mut self, new_ip: &Ipv4Addr) {
        // unsafe { copy_to_sockaddr!(new_ip, self.ifr.ifr_ifru.ifru_dstaddr) };

        copy_ip(new_ip, &mut self.ifr);
        self.call(libc::SIOCSIFDSTADDR, "SIOCSIFDSTADDR");
    }

    pub fn change_bordcast_ip(&mut self, new_ip: &Ipv4Addr) {
        // unsafe { copy_to_sockaddr!(new_ip, self.ifr.ifr_ifru.ifru_broadaddr) };
        // self.call(libc::SIOCSIFBRDADDR, "SIOCSIFBRDADDR");

        copy_ip(new_ip, &mut self.ifr);

        self.call(libc::SIOCSIFBRDADDR, "SIOCSIFBRDADDR");
    }
    pub fn change_netmask_ip(&mut self, new_ip: &Ipv4Addr) {
        copy_ip(new_ip, &mut self.ifr);
        self.call(libc::SIOCSIFNETMASK, "SIOCSIFNETMASK");
    }
    pub fn change_mtu(&mut self, new_mtu: usize) {
        self.ifr.ifr_ifru.ifru_mtu = new_mtu as i32;
        self.call(libc::SIOCSIFMTU, "SIOCSIFMTU");
    }
    pub fn change_queue_len(&mut self, new_mtu: usize) {
        self.ifr.ifr_ifru.ifru_metric = new_mtu as i32;
        self.call(libc::SIOCSIFTXQLEN, "SIOCSIFTXQLEN");
    }
}

#[inline]
fn ip_to_sockaddr_in(ip4: &str) -> sockaddr_in {
    sockaddr_in {
        sin_family: libc::AF_INET as u16,
        sin_port: 0,
        sin_addr: in_addr {
            s_addr: u32::from_be(Ipv4Addr::from_str(ip4).unwrap().into()),
        },
        sin_zero: [0; 8],
    }
}

fn to_sa_data(x: &Ipv4Addr) -> [i8; 14] {
    let mut result = [0i8; 14];
    let o = x.octets();
    for i in 2..6 {
        result[i] = o[i - 2] as i8;
    }
    result
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct MacAddr {
    octets: [u8; 6],
}
impl MacAddr {
    pub const fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        MacAddr {
            octets: [a, b, c, d, e, f],
        }
    }

    #[inline]
    pub const fn octets(&self) -> [u8; 6] {
        self.octets
    }
}

fn copy_mac(new_mac: &str, req: &mut ifreq) {
    let mut mac: [i8; 14] = [0; 14];
    unsafe {
        let mut mac_u8: [u8; 14] = [0; 14];
        for (i, hex_byte) in new_mac.replace(":", "").as_bytes().chunks(2).enumerate() {
            mac_u8[i] = u8::from_str_radix(std::str::from_utf8_unchecked(hex_byte), 16)
                .expect("Invalid MAC");
        }
        std::ptr::copy(mac_u8.as_ptr(), mac.as_mut_ptr() as *mut u8, mac_u8.len());
    }

    req.ifr_ifru.ifru_hwaddr = sockaddr {
        sa_family: libc::ARPHRD_ETHER,
        sa_data: mac,
    };
}

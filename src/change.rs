use std::ffi::CStr;
use std::net::Ipv4Addr;

use crate::defin::MacAddr;
use crate::dev_info::copy_interface;

use libc::{
    __errno_location, ifreq, ioctl, sockaddr, socket, strerror, AF_INET,
    SOCK_DGRAM,
};

#[macro_export]
macro_rules! print_error {
    ($prefix:expr $(, $subfix:expr)?) => {
        let rstr = CStr::from_ptr(strerror(*__errno_location())).to_str().unwrap();
        print!("{}: {}", $prefix, rstr);
        $(print!(" {}", $subfix);)?
    };
}

#[derive(Debug, Clone)]
pub struct InetModify {
    sock: i32,
    ifr: ifreq,
    interface: Option<String>,
}

// macro_rules! to_ptr {
//     ($x:expr) => {
//         //  $x as *mut _ as *mut std::ffi::c_void
//         &($x) as *const _ as *mut std::ffi::c_void
//     };
// }

// macro_rules! sizeof {
//     ($ty:ty) => {
//         std::mem::size_of::<$ty>()
//     };
// }

pub fn ipv4_to_sockaddr(ip: &Ipv4Addr) -> sockaddr {
    let mut addr = [0i8; 14];

    for (idx, _) in ip.octets().iter().enumerate() {
        addr[idx + 2] = ip.octets()[idx] as i8;
    }

    sockaddr {
        sa_family: AF_INET as u16,
        sa_data: addr,
    }
}

impl Default for InetModify {
    fn default() -> Self {
        unsafe {
            InetModify {
                sock: socket(AF_INET, SOCK_DGRAM, 0),
                ifr: std::mem::zeroed(),
                interface: None,
            }
        }
    }
}

impl InetModify {
    pub fn new(interface: &str) -> Self {
        let mut inet = InetModify::default();
        inet.interface = Some(interface.to_string());
        copy_interface(&mut inet.ifr.ifr_name, interface);
        inet
    }

    pub fn set_interface(&mut self, interface: &str) {
        copy_interface(&mut self.ifr.ifr_name, interface);
        self.interface = Some(interface.into());
    }

    fn call(&mut self, req: u64, req_name: &str) {
        unsafe {
            if ioctl(self.sock, req, &mut self.ifr) != 0 {
                print_error!(req_name, "(Suggestion: try with sudo)\n");
                // dbg!(self);
                // std::process::exit(-1)
            }
        }
    }

    pub fn interface_up(&mut self) {
        self.ifr.ifr_ifru.ifru_flags = self.get_flag() | libc::IFF_UP as i16;

        self.call(libc::SIOCSIFFLAGS, "SIOCSIFFLAGS");
    }

    pub fn interface_down(&mut self) {
        self.ifr.ifr_ifru.ifru_flags = self.get_flag() & !libc::IFF_UP as i16;

        self.call(libc::SIOCSIFFLAGS, "SIOCSIFFLAGS");
    }

    pub fn get_flag(&mut self) -> i16 {
        self.call(libc::SIOCGIFFLAGS, "SIOCGIFFLAGS");
        unsafe { self.ifr.ifr_ifru.ifru_flags }
    }

    pub fn rename_interface(&mut self, new_interface: &str) {
        // copy_new_interface(new_interface, &mut self.ifr);
        self.interface_down();
        copy_interface(
            &mut unsafe { self.ifr.ifr_ifru.ifru_newname },
            new_interface,
        );
        self.call(libc::SIOCSIFNAME, "SIOCSIFNAME");
        self.interface_up();
    }

    pub fn change_mac(&mut self, new_mac: &MacAddr) {
        // mac_to_sockaddr
        // copy_mac(new_mac, &mut self.ifr);
        self.ifr.ifr_ifru.ifru_hwaddr = mac_to_sockaddr(new_mac);
        self.call(libc::SIOCSIFHWADDR, "SIOCSIFHWADDR");
    }

    pub fn get_mac(&mut self) -> MacAddr {
        self.call(libc::SIOCGIFHWADDR, "SIOCGIFFLAGS");

        unsafe {
            MacAddr::from_vec(
                &self
                    .ifr
                    .ifr_ifru
                    .ifru_hwaddr
                    .sa_data
                    .iter()
                    .take(6)
                    .map(|&byte| byte as u8)
                    .collect::<Vec<u8>>(),
            )
        }
    }
    pub fn add_ip(&mut self, new_ip: &Ipv4Addr) {
        self.ifr.ifr_ifru.ifru_addr = ipv4_to_sockaddr(new_ip);
        // copy_to_sockaddr!(new_ip, self.ifr.ifr_ifru.ifru_addr);
        self.call(libc::SIOCSIFADDR, "SIOCSIFADDR");
    }
    pub fn get_ip4(&self) -> Vec<Ipv4Addr> {
        // self.ifr.ifr_ifru.ifru_addr = copy_ip4addr_to_sockin(&Ipv4Addr::UNSPECIFIED);

        let mut ifconf: libc::ifconf = unsafe { std::mem::zeroed() };

        let mut buffer: [libc::c_char; 16384] = [0; 16384];

        ifconf.ifc_ifcu.ifcu_req = buffer.as_mut_ptr() as *mut ifreq;
        ifconf.ifc_len = 16384;

        // self.call(libc::SIOCGIFCONF, "SIOCGIFCONF");

        let _ = unsafe { libc::ioctl(self.sock, libc::SIOCGIFCONF, &mut ifconf) };

        let num_interfaces = ifconf.ifc_len / std::mem::size_of::<ifreq>() as i32;

        let mut interfaces = Vec::new();
        for i in 0..num_interfaces {
            let ifr: ifreq = unsafe { *ifconf.ifc_ifcu.ifcu_req.offset(i as isize) };

            let t = String::from_utf8(
                ifr.ifr_name
                    .iter()
                    .filter(|x| **x > 0)
                    .map(|x| *x as u8)
                    .collect(),
            )
            .unwrap();
            if t.eq(self.interface.as_ref().unwrap()) {
                let ip4 = unsafe { ifr.ifr_ifru.ifru_addr.sa_data };

                interfaces.push(Ipv4Addr::new(
                    ip4[2] as u8,
                    ip4[3] as u8,
                    ip4[4] as u8,
                    ip4[5] as u8,
                ));
            }
        }
        interfaces
    }
    pub fn change_dest_ip(&mut self, new_ip: &Ipv4Addr) {
        // unsafe { copy_to_sockaddr!(new_ip, self.ifr.ifr_ifru.ifru_dstaddr) };
        // copy_ip(new_ip, &mut self.ifr);
        self.ifr.ifr_ifru.ifru_dstaddr = ipv4_to_sockaddr(new_ip);
        self.call(libc::SIOCSIFDSTADDR, "SIOCSIFDSTADDR");
    }

    pub fn change_bordcast_ip(&mut self, new_ip: &Ipv4Addr) {
        // unsafe { copy_to_sockaddr!(new_ip, self.ifr.ifr_ifru.ifru_broadaddr) };
        // self.call(libc::SIOCSIFBRDADDR, "SIOCSIFBRDADDR");

        // copy_ip(new_ip, &mut self.ifr);
        self.ifr.ifr_ifru.ifru_broadaddr = ipv4_to_sockaddr(new_ip);

        self.call(libc::SIOCSIFBRDADDR, "SIOCSIFBRDADDR");
    }
    pub fn change_netmask_ip(&mut self, new_ip: &Ipv4Addr) {
        // copy_ip(new_ip, &mut self.ifr);
        self.ifr.ifr_ifru.ifru_netmask = ipv4_to_sockaddr(new_ip);

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



fn mac_to_sockaddr(mac: &MacAddr) -> sockaddr {
    let mut new_mac = [0i8; 14];

    for (idx, val) in mac.octets().iter().enumerate() {
        new_mac[idx] = *val as i8;
    }
    sockaddr {
        sa_family: libc::ARPHRD_ETHER,
        sa_data: new_mac,
    }
}

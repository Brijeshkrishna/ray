use std::ffi::CStr;
use std::ffi::CString;
use std::net::Ipv4Addr;

use crate::defin::MacAddr;
use crate::dev_info::copy_interface;

use libc::ifconf;
use libc::{
    __errno_location, ifreq, ioctl, sockaddr, socket, strerror, AF_INET, IFNAMSIZ, SOCK_DGRAM,
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

pub fn get_previous_error<'a>() -> &'a str {
    unsafe { CStr::from_ptr(strerror(*__errno_location())) }
        .to_str()
        .unwrap()
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

fn interface_c_str(iface: &str) -> [i8; 16] {
    let mut new_name = [0i8; IFNAMSIZ];

    for (idx, value) in CString::new(iface)
        .unwrap()
        .as_bytes_with_nul()
        .iter()
        .enumerate()
    {
        new_name[idx] = *value as i8;
    }
    new_name
}

impl InetModify {
    pub fn new(interface: &str) -> Self {
        let mut ifr: ifreq = unsafe { std::mem::zeroed() };
        ifr.ifr_name = interface_c_str(interface);
        InetModify {
            interface: Some(interface.to_string()),
            ifr,
            sock: unsafe{socket(AF_INET, SOCK_DGRAM, 0)},
        }
    }

    pub fn set_interface_name(&mut self, interface: &str) {
        copy_interface(&mut self.ifr.ifr_name, interface);
        self.interface = Some(interface.into());
    }

    fn call(&self, req: u64, req_name: &str) -> Result<(), &'static str> {
        unsafe {
            if ioctl(self.sock, req, &self.ifr) != 0 {
                // unsafe contdition
                // println!("{}",std::io::Error::last_os_error().raw_os_error().unwrap());
                print_error!(req_name, "(Suggestion: try with sudo)\n");
                return Err(get_previous_error());
                // dbg!(self);
                // std::process::exit(-1)
            }
            Ok(())
        }
    }

    pub fn rename_interface(&mut self, new_interface: &str) -> Result<(), &'static str> {
        if new_interface.len() <= IFNAMSIZ {
            return Err("Interface length is {}can't be greater then 15");
        } else if new_interface.is_empty() {
            return Err("Interface length can't be 0");
        }

        self.interface_down()?;

        let mut new_name = [0i8; IFNAMSIZ];

        for (idx, value) in CString::new(new_interface)
            .unwrap()
            .as_bytes_with_nul()
            .iter()
            .enumerate()
        {
            new_name[idx] = *value as i8;
        }

        self.ifr.ifr_ifru.ifru_newname = new_name;
        // copy_interface(
        //     &mut unsafe { self.ifr.ifr_ifru.ifru_newname },
        //     new_interface,
        // );
        self.call(libc::SIOCSIFNAME, "SIOCSIFNAME")?;
        self.interface_up()?;
        Ok(())
    }

    pub fn change_dest_ip(&mut self, new_ip: &Ipv4Addr) -> Result<(), &'static str> {
        self.ifr.ifr_ifru.ifru_dstaddr = InetModify::ipv4_to_sockaddr(new_ip);
        self.call(libc::SIOCSIFDSTADDR, "SIOCSIFDSTADDR")?;
        Ok(())
    }

    pub fn change_bordcast_ip(&mut self, new_ip: &Ipv4Addr) -> Result<(), &'static str> {
        // unsafe { copy_to_sockaddr!(new_ip, self.ifr.ifr_ifru.ifru_broadaddr) };
        // self.call(libc::SIOCSIFBRDADDR, "SIOCSIFBRDADDR");

        // copy_ip(new_ip, &mut self.ifr);
        self.ifr.ifr_ifru.ifru_broadaddr = InetModify::ipv4_to_sockaddr(new_ip);

        self.call(libc::SIOCSIFBRDADDR, "SIOCSIFBRDADDR")?;
        Ok(())
    }
    pub fn change_netmask_ip(&mut self, new_ip: &Ipv4Addr) -> Result<(), &'static str> {
        // copy_ip(new_ip, &mut self.ifr);
        self.ifr.ifr_ifru.ifru_netmask = InetModify::ipv4_to_sockaddr(new_ip);

        self.call(libc::SIOCSIFNETMASK, "SIOCSIFNETMASK")?;
        Ok(())
    }

    pub fn set_queue_len(&mut self, new_mtu: u32) -> Result<(), &'static str> {
        self.ifr.ifr_ifru.ifru_metric = new_mtu as i32;
        self.call(libc::SIOCSIFTXQLEN, "SIOCSIFTXQLEN")?;
        Ok(())
    }

    pub fn get_queue_len(&mut self) -> Result<u32, &'static str> {
        self.call(libc::SIOCGIFTXQLEN, "SIOCGIFTXQLEN")?;
        Ok(unsafe { self.ifr.ifr_ifru.ifru_metric } as u32)
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

// operation on flags

impl InetModify {
    pub fn interface_up(&mut self) -> Result<(), &'static str> {
        self.set_flag(self.get_flag()? | libc::IFF_UP as i16)?;
        Ok(())
    }

    pub fn interface_down(&mut self) -> Result<(), &'static str> {
        self.set_flag(self.get_flag()? & !libc::IFF_UP as i16)?;
        Ok(())
    }

    pub fn get_flag(&self) -> Result<i16, &'static str> {
        self.call(libc::SIOCGIFFLAGS, "SIOCGIFFLAGS")?;
        Ok(unsafe { self.ifr.ifr_ifru.ifru_flags })
    }

    pub fn set_flag(&mut self, flag: i16) -> Result<(), &'static str> {
        self.ifr.ifr_ifru.ifru_flags = flag;
        self.call(libc::SIOCSIFFLAGS, "SIOCSIFFLAGS")?;
        Ok(())
    }
}

// operation on MAC
impl InetModify {
    pub fn set_mac(&mut self, new_mac: &MacAddr) -> Result<(), &'static str> {
        // mac_to_sockaddr
        // copy_mac(new_mac, &mut self.ifr);
        self.ifr.ifr_ifru.ifru_hwaddr = mac_to_sockaddr(new_mac);
        self.call(libc::SIOCSIFHWADDR, "SIOCSIFHWADDR")?;
        Ok(())
    }

    pub fn get_mac(&mut self) -> Result<MacAddr, &'static str> {
        self.call(libc::SIOCGIFHWADDR, "SIOCGIFFLAGS")?;

        let mut octets = [0u8; 6];

        for (idx, val) in unsafe { self.ifr.ifr_ifru.ifru_hwaddr.sa_data }
            .iter()
            .enumerate()
        {
            octets[idx] = *val as u8;
        }

        Ok(MacAddr::from_octets(octets))
    }
}

impl InetModify {
    fn ipv4_to_sockaddr(ip: &Ipv4Addr) -> sockaddr {
        let mut addr = [0i8; 14];
    
        for (idx, _) in ip.octets().iter().enumerate() {
            addr[idx + 2] = ip.octets()[idx] as i8;
        }
        sockaddr {
            sa_family: AF_INET as u16,
            sa_data: addr,
        }
    }
    
    
}
// IPV4 operations

impl InetModify {
    pub fn add_ip(&mut self, new_ip: &Ipv4Addr) -> Result<(), &'static str> {
        self.ifr.ifr_ifru.ifru_addr = InetModify::ipv4_to_sockaddr(new_ip);
        self.call(libc::SIOCSIFADDR, "SIOCSIFADDR")?;
        Ok(())
    }

    fn call_ifconf(&self) -> Result<ifconf, &'static str> {
        let mut ifc: ifconf = unsafe { std::mem::zeroed() };

        let mut buffer: [libc::c_char; 4096] = [0; 4096];

        ifc.ifc_ifcu.ifcu_req = buffer.as_mut_ptr() as *mut ifreq;
        ifc.ifc_len = std::mem::size_of_val(&buffer) as i32;

        if unsafe { libc::ioctl(self.sock, libc::SIOCGIFCONF, &mut ifc) } != 0 {
            return Err(get_previous_error());
        }
        Ok(ifc)
    }

    pub fn get_ip4(&self) -> Result<Vec<Ipv4Addr>, &'static str> {
        let ifc = self.call_ifconf()?;

        let num_iface = ifc.ifc_len / std::mem::size_of::<ifreq>() as i32;

        let mut interfaces = Vec::new();

        for i in 0..num_iface {
            let ifr: ifreq = unsafe { *ifc.ifc_ifcu.ifcu_req.offset(i as isize) };

            let iface = unsafe { CString::from_raw(ifr.ifr_name.as_ptr() as *mut i8) };

            if iface
                .as_c_str()
                .to_str()
                .unwrap()
                .eq(self.interface.as_ref().unwrap())
            {
                interfaces.push(InetModify::sockaddr_to_ipv4(&unsafe { ifr.ifr_ifru.ifru_addr }));
            }
        }
        Ok(interfaces)
    }

    fn sockaddr_to_ipv4(sock_ip: &sockaddr) -> Ipv4Addr {
        Ipv4Addr::new(
            sock_ip.sa_data[2] as u8,
            sock_ip.sa_data[3] as u8,
            sock_ip.sa_data[4] as u8,
            sock_ip.sa_data[5] as u8,
        )
    }

}

// MTU operations

impl InetModify {
    pub fn set_mtu(&mut self, new_mtu: u32) -> Result<(), &'static str> {
        self.ifr.ifr_ifru.ifru_mtu = new_mtu as i32;
        self.call(libc::SIOCSIFMTU, "SIOCSIFMTU")?;
        Ok(())
    }

    pub fn get_mtu(&self) -> Result<i32, &'static str> {
        self.call(libc::SIOCGIFMTU, "SIOCGIFMTU")?;
        Ok(unsafe { self.ifr.ifr_ifru.ifru_mtu })
    }
}

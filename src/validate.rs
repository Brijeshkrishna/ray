use std::str::FromStr;

use libc::*;
use crate::{defin::MacAddr, dev_info::copy_interface};

pub fn valid_existing_interface(interface: &str) -> Result<String, String> {
    if interface.len() > IFNAMSIZ {
        return Err("Interface name is too long (>16)".into());
    }
    if ! interface_exist(interface) {
        return Err("No such interface found".into());
    }
    Ok(interface.into())
}

pub fn valid_new_interface(interface: &str) -> Result<String, String>  {

    if interface.len() > IFNAMSIZ {
        return Err("Interface name is too long (>16)".into());
    }
    Ok(interface.into())   
}

pub fn interface_exist(interface: &str) -> bool {
    unsafe {
        let sock = socket(AF_INET, SOCK_DGRAM, 0);
        let mut req: ifreq = std::mem::zeroed();
        copy_interface(&mut req.ifr_name, interface);

        if libc::ioctl(sock, SIOCGIFFLAGS, &mut req) == -1 {
            return false;
        }
        true 
    }
}

pub fn is_valid_mac(mac: &str) -> Result<String, String> {
    
    match MacAddr::from_str(mac){
        Ok(_) => Ok(mac.to_string()),
        _ => Err("Invaild MAC address".to_string())
    }
}
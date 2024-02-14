use libc::*;
use crate::dev_info::copy_interface;

pub fn valid_existing_interface(interface: &str) -> Result<String, String> {
    if interface.len() > IFNAMSIZ {
        return Err("Interface name is too long (>16)".into());
    }
    if ! interface_exist(interface) {
        return Err("No such interface found".into());
    }
    return Ok(interface.into());
}

pub fn valid_new_interface(interface: &str) -> Result<String, String>  {

    if interface.len() > IFNAMSIZ {
        return Err("Interface name is too long (>16)".into());
    }
    return Ok(interface.into());    
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
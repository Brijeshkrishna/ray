use nix::libc::*;


macro_rules! print_error {
    ($from:expr) => {
        let r = strerror(*libc::__errno_location());
        let rstr = std::ffi::CStr::from_ptr(r).to_str().unwrap();
        println!("{}: {rstr}",$from);
    };
}

pub fn change_mac(interface : &str, new:&str){

    let socket_fd = unsafe {
        libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0)
    };

    let mut ifreq: libc::ifreq =  unsafe { std::mem::zeroed() };



    let mut mac:[i8;14] = [0;14];
    unsafe{

        std::ptr::copy(interface.as_ptr(),ifreq.ifr_name.as_mut_ptr() as *mut u8, interface.len());
        let mut mac_u8: [u8; 14] = [0; 14];
        for (i, hex_byte) in new.as_bytes().chunks(2).enumerate() {
            mac_u8[i] = u8::from_str_radix(std::str::from_utf8_unchecked(hex_byte), 16).unwrap();
        }
        std::ptr::copy(mac_u8.as_ptr(), mac.as_mut_ptr() as *mut u8, mac_u8.len());
    }

    ifreq.ifr_ifru.ifru_hwaddr = sockaddr{sa_family:ARPHRD_ETHER,sa_data:mac};
    unsafe { 
    
    ioctl(socket_fd, SIOCSIFHWADDR, &mut ifreq) ; 
        print_error!("dad");
           

            

    }
    

}


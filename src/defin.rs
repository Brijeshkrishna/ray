use std::net::{IpAddr, Ipv4Addr};


#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug, PartialOrd, Ord)]
pub struct MacAddr {
    octets: [u8; 6],
}
impl MacAddr {

    pub const UNSPECIFIED: Self = MacAddr { octets: [0 ; 6] };

    pub const fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        MacAddr {
            octets: [a, b, c, d, e, f],
        }
    }

    pub const fn from_be(value: [u8; 6]) -> Self {
        MacAddr { octets: value }
    }

    pub fn from_vec(value: &Vec<u8>) -> Self {
        let mut mac = MacAddr::default();
        mac.octets.copy_from_slice(&value[0..6]);
        mac
    }

    #[inline]
    pub const fn octets(&self) -> [u8; 6] {
        self.octets

    }

    pub fn to_string(&self) -> String {
        self.octets
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(":")
    }

    pub fn to_hex_string(&self) -> String {
        self.octets
            .iter()
            .map(|x| format!("{:X}", x))
            .collect::<Vec<String>>()
            .join(":")
    }
    pub fn from_str(mac_str: &str)->Self{
        let mut octets =  [0u8; 6];

        for (idx, hex_byte) in mac_str.replace(":", "").as_bytes().chunks(2).enumerate() {
            octets[idx] = u8::from_str_radix(unsafe { std::str::from_utf8_unchecked(hex_byte) }, 16)
                .expect("Invalid MAC");
        }
        MacAddr::from_be(octets)
    }
    // fn copy_mac(new_mac: &str, req: &mut ifreq) {
    //     let mut mac: [i8; 14] = [0; 14];
    //     unsafe {
    //         let mut mac_u8: [u8; 14] = [0; 14];
    //         for (i, hex_byte) in new_mac.replace(":", "").as_bytes().chunks(2).enumerate() {
    //             mac_u8[i] = u8::from_str_radix(std::str::from_utf8_unchecked(hex_byte), 16)
    //                 .expect("Invalid MAC");
    //         }
    //         std::ptr::copy(mac_u8.as_ptr(), mac.as_mut_ptr() as *mut u8, mac_u8.len());
    //     }

    //     req.ifr_ifru.ifru_hwaddr = sockaddr {
    //         sa_family: libc::ARPHRD_ETHER,
    //         sa_data: mac,
    //     };
    // }
}



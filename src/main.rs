mod change;
mod info;
use std::{
    borrow::Cow,
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};
mod defin;
use change::*;
mod dev;
use colored::Style;
use dev::*;
use libc::{ifreq, AF_INET, SOCK_DGRAM};

use hashbrown::HashMap;
use reqwest::Response;

use crate::{
    dev::{get, Receive, Transmit},
    info::copy_interface,
};

mod dev_info;
use dev_info::*;

use tabled::{
    builder::Builder,
    col,
    grid::config::AlignmentHorizontal,
    row,
    settings::{
        object::{Column, Columns, Segment},
        style::*,
        themes::{Colorization, ColumnNames},
        Alignment, Color, Modify, *,
    },
    Tabled,
};

mod all;

#[tokio::main]
async fn main() {
    // info::info();
    // let a = dev::mai();
    // println!("{:?}",a);

    // use tabled::{
    //     builder::Builder,
    //     settings::{style::*, themes::{Colorization,ColumnNames}, Color,Modify, Alignment, object::Segment,*},
    //     row,col
    // };

    // let mut table  = Table::new(a);
    // table.with(Style::modern().corner_bottom_left('╰').corner_bottom_right('╯').corner_top_left('╭').corner_top_right('╮'));
    // table.with(Modify::new(Segment::all()).with(Alignment::center_vertical()));
    // // table.with(Rotate::Left);
    // table.with(ColumnNames::default().set_alignment(tabled::grid::config::AlignmentHorizontal::Left));

    // println!("{}",Table::new(dev::parse_wireless_interfaces()));

    // println!("{}",table.to_string());

    // addrs("wlp3s0");
    // change_name("wlp5s0f4u2","wlp3s1");
    // net_up("wlp3s1");
    // info::info();

    // change::change_mac("enp2s0", "96d8d983f823");

    // let mut ifreq: libc::ifreq =  unsafe { std::mem::zeroed() };

    // let mut ifc: ifconf =  ifconf { ifc_len: size_of::<ifreq>() as i32 , ifc_buf: &mut ifreq };

    // let socket_fd = unsafe {
    //     libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0)
    // };

    // unsafe{ioctl(socket_fd, SIOCGIFCONF, &ifc);}
    // unsafe{change::print_error!("ioctl");}

    // print!("{:?}",ifreq.ifr_name);

    //     let sock: c_int;

    //     let mut ifr: [libc::ifreq;10] =  unsafe { std::mem::zeroed() };
    //     let  ifc:ifconf  = ifconf { ifc_len: 400 as c_int , ifc_req: ifr.as_mut_ptr() };

    //     println!("{:?}",ifc);

    //     // ifc.ifc_len = 400;
    //     // ifc.ifc_req = ifr.as_mut_ptr();

    //     sock = unsafe {
    //         socket(AF_INET, SOCK_DGRAM, 0)
    //    };

    //     if ( unsafe{ioctl(sock, SIOCGIFCONF, &ifc)} <0) {
    //         unsafe{print_error!("ioctl");}
    //     }

    //     println!("{:?}",ifr[0].ifr_name);

    //     let mut buf = Vec::with_capacity(1500);
    //     ifc.ifc_buf = buf.as_mut_ptr();
    //     ifc.ifc_len = 150;

    //     if ioctl(sock, SIOCGIFCONF, &mut ifc) < 0 {
    //         change::print_error!("ioctl");
    //         return;
    //     }

    //     let mut ifreq: libc::ifreq =  unsafe { std::mem::zeroed() };

    //     let count =( ifc.ifc_len as usize / std::mem::size_of::<libc::ifreq>())as libc::c_ulong;

    //     let mut e = std::mem::size_of::<libc::ifreq>() ;
    //     for _ in 0..count {
    //         unsafe{ std::ptr::copy(ifc.ifc_buf,ifreq.ifr_name.as_mut_ptr() as *mut i8,e) }
    //         e+=e;

    //         let name = std::ffi::CString::from_raw(ifreq.ifr_name.as_mut_ptr() as *mut i8).to_owned();
    //         println!("Device: {:?}", name);
    //     }
    // }

    // let mut map = HashMap::new();

    // let (response_ipv4, response_ipv6) = tokio::join!(
    //     reqwest::get("https://ipinfo.io/ip"),
    //     reqwest::get("https://ifconfig.me/ip")
    // );

    // let rv =  Device::new(&"wlp3s0".to_string()).unwrap() ;

    // d8:c0:a6:57:73:8b

    #[derive(Copy, Clone, PartialEq, Eq, Hash,Default)]
    pub struct MacAddr {
        octets: [u8; 6],
    }
    impl MacAddr {
        pub const fn new(a: u8, b: u8, c: u8, d: u8,e:u8,f:u8) -> Self {
            MacAddr { octets: [a, b, c, d,e,f] }
        }
    
        #[inline]
        pub const fn octets(&self) -> [u8; 6] {
            self.octets
        }
        
    }
    println!("{:?}",dev_info::display("wlp5s0f3u3"));
    // info::info();

        let mut  i = change::InetModify::new("wlp5s0f3u3").unwrap();
        i.change_bordcast_ip(&Ipv4Addr::new(255, 1, 255, 0));
        // let a = MacAddr::new(0xff,0xc0,0xa6,0x57,0x73,0x8b);
        // let mut f =[0i8;14];
        // let b :[i8;6] = a.octets().iter().map(|x| *x as i8 ).collect::<Vec<i8>>().try_into().unwrap();
        // f[0..6].copy_from_slice(&b);


        // let new_mac = "d8:c0:a6:57:73:8b";

        // let mut mac: [i8; 14] = [0; 14];

        // unsafe {
        //     let mut mac_u8: [u8; 14] = [0; 14];
        //     for (i, hex_byte) in new_mac.replace(":", "").as_bytes().chunks(2).enumerate() {
        //         mac_u8[i] = u8::from_str_radix(std::str::from_utf8_unchecked(hex_byte), 16)
        //             .expect("Invalid MAC");
        //     }
        //     std::ptr::copy(mac_u8.as_ptr(), mac.as_mut_ptr() as *mut u8, mac_u8.len());
        //     println!("{:?} {:?}",mac , f);

        // }




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

        let a = "19";
        

        // display("wlp3s0");
        // get_all_ip("wlp3s0");
    // let mut a = change::setter::new("wlp3s0");
    // a.interface_down();
    // a.change_queue_len(1500);
    // // a.change_interface_name("wlp3s0");
    // a.change_mac("96:d8:d9:83:f8:2o");
    // a.interface_up();
    /*

    feat: Added ifconf struct
    Adding ifconf struct
    
    As per defined in C

    ```c
    struct ifconf
      {
        int	ifc_len;			/* Size of buffer.  */
        union
          {
        __caddr_t ifcu_buf;
        struct ifreq *ifcu_req;
          } ifc_ifcu;
      };
    ```
     */

    //     println!(
    //     "
    // Interface: \x1b[1;32m{}\x1b[0m
    // Flags:     {} < {}>

    // {}

    // {}

    // {}

    // {}

    // {}

    // ",
    //         rv.name,
    //         rv.flag,
    //         style_flag(rv.flag),
    //         build_ip4(&rv),
    //         build_ip6(&ip6),
    //         build_hard_info(&rv),
    //         build_rx_tx(&rv),
    //         build_wireless(&rv)
    //     );

    // // let public_ipv4 = response_ipv4.unwrap().text().await.unwrap();
    // let public_ipv6 = response_ipv6.unwrap().text().await.unwrap();

    // println!("{} {} ",public_ipv4,public_ipv6);
    

}

fn bit_to_string(bytes: f64) -> String {
    let mut index: u8 = 1;
    let mut bytes = bytes;

    while bytes > 1024.0 && index <= 7 {
        bytes = bytes / 1024.0;
        index += 1;
    }
    format!(
        "\u{1b}[0m (\u{1b}[36m {} {}\u{1b}[0m )",
        (bytes * 100.0).round() / 100.0,
        match index {
            1 => "bytes",
            2 => "KB",
            3 => "MB",
            4 => "GB",
            5 => "TB",
            6 => "EB",
            7 => "ZB",
            8 => "YB",
            _ => "bytes",
        }
        .to_string(),
    )
}
//ioctl(3, SIOCSIFHWADDR, {ifr_name="enp2s0", ifr_hwaddr={sa_family=ARPHRD_ETHER, sa_data=96:78:d9:83:f8:21}}) = 0
//ioctl(3, SIOCSIFHWADDR, {ifr_name="enp2s0", ifr_hwaddr={sa_family=ARPHRD_ETHER, sa_data=65:6e:70:32:73:30}}) = -1

// /// This style looks like a [`Style::modern`] but with horizontal lines.
// ///
// /// Beware: It uses UTF-8 characters.
// ///
// /// ```text
// ///     ╭────┬──────────────┬───────────────────────────╮
// ///     │ id │ destribution │           link            │
// ///     ├────┼──────────────┼───────────────────────────┤
// ///     │ 0  │    Fedora    │  https://getfedora.org/   │
// ///     ├────┼──────────────┼───────────────────────────┤
// ///     │ 2  │   OpenSUSE   │ https://www.opensuse.org/ │
// ///     ├────┼──────────────┼───────────────────────────┤
// ///     │ 3  │ Endeavouros  │ https://endeavouros.com/  │
// ///     ╰────┴──────────────┴───────────────────────────╯
// /// ```
// pub const fn cool() -> Style<On, On, On, On, On, On> {
//     Style::new(
//         create_borders(
//             Line::full('─', '┬', '╭', '╮'),
//             Line::full('─', '┴', '╰', '╯'),
//             Line::full('─', '┼', '├', '┤'),
//             Some('│'),
//             Some('│'),
//             Some('│'),
//         ),
//         [HorizontalLine::new(1, Line::full('─', '┼', '├', '┤'))],
//         [],
//     )
// }

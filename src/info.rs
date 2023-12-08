use colored::{self, Colorize};
use libc::*;
use std::fs::File;
use std::io::BufReader;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::{collections::HashMap, io::BufRead};

use tabled::{
    builder::Builder,
    col, row,
    settings::{
        object::Segment,
        style::*,
        themes::{Colorization, ColumnNames},
        Alignment, Color, Modify, *,
    },
};

fn getip6(interface: &str) -> String {
    let addr = std::fs::read_to_string("/proc/net/if_inet6").unwrap();
    let values: Vec<&str> = addr.split_whitespace().collect();

    for i in (0..values.len()).step_by(6) {
        if values[i + 5].eq(interface) {
            // values[i].to_string().insert_str(4, ":");
            // values[i].to_string().insert_str(9, ":");
            // values[i].to_string().insert_str(14, ":");
            // values[i].to_string().insert_str(19, ":");
            // values[i].to_string().insert_str(24, ":");
            // values[i].to_string().insert_str(29, ":");
            // values[i].to_string().insert_str(35, ":");
            return values[i].to_string();
        }
    }
    return "-".to_string();
}
fn get_flag(interface: &str) -> i16 {
    let socket_fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0) };

    let mut ifreq: libc::ifreq = unsafe { std::mem::zeroed() };

    unsafe {
        std::ptr::copy(
            interface.as_ptr(),
            ifreq.ifr_name.as_mut_ptr() as *mut u8,
            interface.len(),
        );
    }

    unsafe {
        libc::ioctl(socket_fd, libc::SIOCGIFFLAGS, &mut ifreq);
    }
    unsafe { ifreq.ifr_ifru.ifru_flags }
}
#[inline]
pub fn copy_interface(req: &mut ifreq, interface: &str) {
    unsafe {
        std::ptr::copy(
            interface.as_ptr(),
            req.ifr_name.as_mut_ptr() as *mut u8,
            interface.len(),
        );
    }
}

pub fn getmac(sock: i32, interface: &str) -> String {
    let mut ifr: libc::ifreq = unsafe { std::mem::zeroed() };



    unsafe {
        std::ptr::copy(
            interface.as_ptr(),
            ifr.ifr_name.as_mut_ptr() as *mut u8,
            interface.len(),
        );
    }

    println!("FFF {:?}",ifr);

    unsafe {
        ioctl(sock, SIOCGIFHWADDR, &mut ifr);

        ifr
            .ifr_ifru
            .ifru_hwaddr
            .sa_data
            .iter()
            .take(6)
            .map(|&byte| format!("{:02X}", byte as u8))
            .collect::<Vec<String>>()
            .join(":")
    }
}

#[inline]
fn get_mac(x: [i8; 14]) -> String {
    x.iter()
        .take(6)
        .map(|&byte| format!("{:02X}", byte as u8))
        .collect::<Vec<String>>()
        .join(":")
}

pub fn addrs(interface: &str) -> String {
    let socket_fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0) };

    let mut ifreq: libc::ifreq = unsafe { std::mem::zeroed() };

    unsafe {
        std::ptr::copy(
            interface.as_ptr(),
            ifreq.ifr_name.as_mut_ptr() as *mut u8,
            interface.len(),
        );
    }

    unsafe {
        libc::ioctl(socket_fd, libc::SIOCGIFADDR, &mut ifreq);
    }
    let flags = unsafe { ifreq.ifr_ifru.ifru_addr.sa_data };

    let ip_addr = Ipv4Addr::new(
        flags[2] as u8,
        flags[3] as u8,
        flags[4] as u8,
        flags[5] as u8,
    );
    ip_addr.to_string()
}

fn get_sym(dev: &str) -> (String, String, String) {
    //     󰈀
    //   777  ifconfig
    //   778  
    //   779  exa
    //   780  exa --cons
    //   781  exa --icons
    //   782  󰦛
    let mut rv = String::from(dev);
    let mut a = "-".to_string();
    // let flag = usize::from_str_radix(std::fs::read_to_string(format!("/sys/class/net/{dev}/flags")).unwrap().trim()[2..].as_ref(),16).unwrap();
    let flag = get_flag(dev) as i32;
    let mut ip6 = "-".to_string();

    if (flag & libc::IFF_UP != 0) {
        rv.insert_str(0, "\x1b[92m ");
    } else {
        rv.insert_str(0, "\x1b[91m ");
    }
    if (flag & libc::IFF_RUNNING != 0) {
        rv.push_str("");
        a = addrs(dev);
        ip6 = getip6(dev);
    }
    rv.push(' ');

    if (flag & libc::IFF_LOOPBACK != 0) {
        rv.push('󰦛');
    } else {
        if std::fs::metadata(format!("/sys/class/net/{dev}/wireless")).is_ok() {
            rv.push('');
        } else {
            rv.push('󰈀');
        }
    }

    (rv, a, ip6)
}

pub fn info() {
    let mut builder = Builder::default();
    builder.set_default_text("─");
    builder.set_header(["┐Interface┌", "┐MAC┌", "┐IP┌", "┐Vendor┌"]);
    let sock = unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0) };

    let file = File::open("/home/brijesh/Projects/cmac/oui").unwrap();
    let reader = BufReader::new(file);

    let mut v: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let d = line.unwrap();
        let d: Vec<&str> = d.split('|').collect();
        v.insert(d.get(0).unwrap().to_string(), d.get(1).unwrap().to_string());
    }

    let paths = std::fs::read_dir("/sys/class/net/").unwrap();

    for path in paths {
        let path = path.as_ref().unwrap();
        // let addr = std::fs::read_to_string(format!("{}/address", path.path().display()))
        //     .unwrap()
        //     .as_str()
        //     .trim()
        //     .to_string();

        // println!("{:?}",path.file_name());
        let addr = getmac(sock, path.file_name().to_owned().to_str().unwrap());
        let k = (addr.as_str())[0..8].to_string().to_uppercase();

        let aa = v.get(&k).unwrap_or(&"─".to_string()).to_owned();

        let temp = get_sym(path.file_name().to_str().unwrap());

        // if temp.1.eq("-"){
        builder.push_record([
            temp.0.as_str(),
            format!(
                "\x1b[93m{}\x1b[0m\x1b[36m{}",
                k,
                (addr.as_str())[8..].to_string()
            )
            .as_str(),
            temp.1.as_str(),
            aa.as_str(),
        ]);

        // }
        // else{
        //     builder.push_record([
        //         temp.0.as_str(),
        //         format!(
        //             "\x1b[93m{}\x1b[0m\x1b[36m{}",
        //             k,
        //             (addr.as_str())[8..].to_string()
        //         )
        //         .as_str(),
        //         col![temp.1,temp.2].with(Style::rounded()).to_string().as_str(),
        //         aa.as_str(),
        //     ]);
        // }
    }
    // //
    // Line::full('─', '┬', '╭', '╮'),
    // Line::full('─', '┴', '╰', '╯'),
    let mut table = builder.build();
    table.with(Style::rounded().remove_horizontals());

    table.with(Colorization::columns([
        Color::FG_BLUE,
        Color::FG_CYAN,
        Color::FG_MAGENTA,
        Color::FG_BLUE,
    ]));

    table.with(Modify::new(Segment::all()).with(Alignment::center_vertical()));
    table.with(ColumnNames::default().set_color(Color::BOLD));

    // table.with(Modify::new(object::Columns::new(3..)).with(Width::wrap(15).keep_words()));
    // table.with(Width::justify(10));
    // table.with(Rotate::Left);

    println!("{}", table);
    // println!("{:?}", unsafe { getmac("wlp3s0") });
}

use crate::defin::MacAddr;

use std::{
    fs,
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use libc::*;
use tabled::{
    builder::Builder,
    grid::config::AlignmentHorizontal,
    settings::{
        themes::{Colorization, ColumnNames},
        Alignment, Color, Style,
    },
    Table,
};

#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct Transmit {
    bytes: usize,
    packets: usize,
    errs: usize,
    drop: usize,
    fifo: usize,
    colls: usize,
    carrier: usize,
    compressed: usize,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct Receive {
    bytes: usize,
    packets: usize,
    errs: usize,
    drop: usize,
    fifo: usize,
    frame: usize,
    compressed: usize,
    multicast: usize,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct WirelessInterface {
    pub link: f64,
    pub level: f64,
    pub noise: isize,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Device {
    interface: String,
    addr: Vec<Ipv4Addr>,
    dstaddr: Ipv4Addr,
    broadaddr: Ipv4Addr,
    netmask: Ipv4Addr,
    hwaddr: MacAddr,
    flag: i16,
    index: i32,
    mtu: i32,
    tr: Transmit,
    rx: Receive,
    qlen: i32,
    wireless: Option<WirelessInterface>,
}

// implement for netmask multi
pub fn get_all_ip(interface: &str) -> Vec<Ipv4Addr> {
    let mut ifconf: ifconf = unsafe { std::mem::zeroed() };

    let mut buffer: [c_char; 16384] = [0; 16384];

    ifconf.ifc_ifcu.ifcu_req = buffer.as_ptr() as *mut ifreq;
    ifconf.ifc_len = 16384;

    let socket_fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_STREAM, 0) };
    unsafe { libc::ioctl(socket_fd, libc::SIOCGIFCONF, &mut ifconf) };

    let num_interfaces = ifconf.ifc_len / std::mem::size_of::<ifreq>() as i32;

    let mut interfaces = Vec::new();
    for i in 0..num_interfaces {
        let ifr: ifreq = unsafe { *ifconf.ifc_ifcu.ifcu_req.offset(i as isize) };
        // println!("==={:?}", unsafe { ifr.ifr_ifru.ifru_data });
        let a = unsafe { parse_ipv4(&ifr.ifr_ifru.ifru_addr.sa_data) };

        let t = String::from_utf8(
            ifr.ifr_name
                .iter()
                .filter(|x| **x > 0)
                .map(|x| *x as u8)
                .collect(),
        )
        .unwrap();
        if t.eq(interface) {
            interfaces.push(a);
        }
    }

    interfaces
}

pub fn display(interface: &str) {
    let rv = Device::new(interface).unwrap();

    let mut builder = Builder::new();

    builder.push_record([build_ip4(&rv)]);
    builder.push_record([build_ip6(
        &getipv6(&rv.interface)
            .iter()
            .map(|x| x.to_string())
            .collect(),
    )]);
    builder.push_record([build_hard_info(&rv)]);
    builder.push_record([build_rx_tx(&rv)]);

    if let Some(y) = build_wireless(&rv) {
        builder.push_record([y]);
    }

    println!(
        "{}",
        builder
            .build()
            .with(Style::modern().remove_horizontal())
            .with(
                ColumnNames::new([format!("─┐ {} ┌", rv.interface)])
                    .set_color(Color::BOLD | Color::FG_GREEN)
                    .set_alignment(AlignmentHorizontal::Center)
            )
    )
}

#[inline]
pub fn copy_interface(req: &mut [c_char; IFNAMSIZ], interface: &str) {
    unsafe {
        std::ptr::write_bytes(req.as_mut_ptr() as *mut u8, 0, IFNAMSIZ);
        std::ptr::copy_nonoverlapping(
            interface.as_ptr(),
            req.as_mut_ptr() as *mut u8,
            interface.len().min(IFNAMSIZ - 1),
        );
    }
}

// #[inline]
// pub  fn copy_interface(req: &mut ifreq, interface: &str) {
//     unsafe{std::ptr::copy(
//         std::ffi::CString::new(interface)
//             .expect("Invalid interface name")
//             .as_ptr(),
//         req.ifr_name.as_mut_ptr() as *mut std::os::raw::c_char,
//         std::cmp::min(interface.as_bytes().len() + 1, IFNAMSIZ - 1),
//     );}
// }

#[inline]
fn parse_ipv4(x: &[i8; 14]) -> Ipv4Addr {
    let mut ip = x.iter().skip(2).take(4).map(|&byte| (byte as u8));
    Ipv4Addr::new(
        ip.next().unwrap(),
        ip.next().unwrap(),
        ip.next().unwrap(),
        ip.next().unwrap(),
    )
}

#[inline]
fn parse_mac(x: &[i8; 14]) -> MacAddr {
    let mut ip = x.iter().take(6).map(|&byte| (byte as u8));
    MacAddr::new(
        ip.next().unwrap(),
        ip.next().unwrap(),
        ip.next().unwrap(),
        ip.next().unwrap(),
        ip.next().unwrap(),
        ip.next().unwrap(),
    )
}

pub fn get_wireless(interface: &str) -> Option<WirelessInterface> {
    let file = std::fs::read_to_string("/proc/net/wireless")
        .expect("Error in reading file /proc/net/wireless");

    for line in file.lines().skip(2) {
        let segments = line.split_whitespace().collect::<Vec<&str>>();
        let piece = segments.as_slice();

        if piece[0].starts_with(interface) {
            return Some(WirelessInterface {
                link: piece[2].parse().unwrap_or_default(),
                level: piece[3].parse().unwrap_or_default(),
                noise: piece[4].parse().unwrap_or_default(),
            });
        };
    }
    None
}

pub fn getipv6(interface: &String) -> Vec<Ipv6Addr> {
    let file =
        fs::read_to_string("/proc/net/if_inet6").expect("Error in reading file /proc/net/if_inet6");

    file.split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(6)
        .filter(|chunk| chunk[5].eq(interface.as_str()))
        .map(|chunk| {
            let ip = chunk[0].to_string();

            Ipv6Addr::new(
                u16::from_str_radix(&ip[..4], 16).unwrap(),
                u16::from_str_radix(&ip[4..8], 16).unwrap(),
                u16::from_str_radix(&ip[8..12], 16).unwrap(),
                u16::from_str_radix(&ip[12..16], 16).unwrap(),
                u16::from_str_radix(&ip[16..20], 16).unwrap(),
                u16::from_str_radix(&ip[20..24], 16).unwrap(),
                u16::from_str_radix(&ip[24..28], 16).unwrap(),
                u16::from_str_radix(&ip[28..32], 16).unwrap(),
            )
        })
        .collect()
    // .map(|chunk| {
    //     chunk[0]
    //         .chars()
    //         .collect::<Vec<char>>()
    //         .chunks(4)
    //         .map(|chunk| chunk.iter().collect::<String>())
    //         .collect::<Vec<String>>()
    //         .join(":")
    // })
    // .collect()
}

fn getdev(interface: &str) -> Option<(Receive, Transmit)> {
    let file =
        std::fs::read_to_string("/proc/net/dev").expect("Error in reading file /proc/net/dev");

    for line in file.lines().skip(2) {
        let mut intf = line.split_whitespace();
        if intf.next().unwrap_or_default().starts_with(interface) {
            let mut parse = || intf.next()?.parse().ok();

            return Some((
                Receive {
                    bytes: parse()?,
                    packets: parse()?,
                    errs: parse()?,
                    drop: parse()?,
                    fifo: parse()?,
                    frame: parse()?,
                    compressed: parse()?,
                    multicast: parse()?,
                },
                Transmit {
                    bytes: parse()?,
                    packets: parse()?,
                    errs: parse()?,
                    drop: parse()?,
                    fifo: parse()?,
                    colls: parse()?,
                    carrier: parse()?,
                    compressed: parse()?,
                },
            ));
        }
    }
    None
}
impl Default for Device {
    fn default() -> Self {
        Self {
            interface: String::default(),
            addr: Vec::default(),
            dstaddr: Ipv4Addr::UNSPECIFIED,
            broadaddr: Ipv4Addr::UNSPECIFIED,
            netmask: Ipv4Addr::UNSPECIFIED,
            hwaddr: MacAddr::default(),
            flag: i16::default(),
            index: i32::default(),
            mtu: i32::default(),
            tr: Transmit::default(),
            rx: Receive::default(),
            qlen: 0,
            wireless: None,
        }
    }
}

impl Device {
    pub fn new(interface: &str) -> Result<Self, ()> {
        let mut rv = Device::default();

        let ioctl_codes: [libc::c_ulong; 9] = [
            libc::SIOCGIFFLAGS,
            libc::SIOCGIFBRDADDR,
            libc::SIOCGIFDSTADDR,
            libc::SIOCGIFNETMASK,
            libc::SIOCGIFADDR,
            libc::SIOCGIFMTU,
            libc::SIOCGIFINDEX,
            libc::SIOCGIFTXQLEN,
            libc::SIOCGIFHWADDR,
        ];

        unsafe {
            let sock = socket(AF_INET, SOCK_DGRAM, 0);

            let mut req: ifreq = std::mem::zeroed();
            copy_interface(&mut req.ifr_name, interface);

            for &code in ioctl_codes.iter() {
                libc::ioctl(sock, code, &mut req);
                match code {
                    libc::SIOCGIFFLAGS => rv.flag = req.ifr_ifru.ifru_flags,
                    libc::SIOCGIFBRDADDR => {
                        rv.broadaddr = parse_ipv4(&req.ifr_ifru.ifru_broadaddr.sa_data)
                    }
                    libc::SIOCGIFDSTADDR => {
                        rv.dstaddr = parse_ipv4(&req.ifr_ifru.ifru_dstaddr.sa_data)
                    }
                    libc::SIOCGIFNETMASK => {
                        rv.netmask = parse_ipv4(&req.ifr_ifru.ifru_netmask.sa_data)
                    }
                    // libc::SIOCGIFADDR => rv.addr = get_ip(req.ifr_ifru.ifru_addr.sa_data),
                    libc::SIOCGIFADDR => rv.addr = get_all_ip(interface),
                    libc::SIOCGIFMTU => rv.mtu = req.ifr_ifru.ifru_mtu,
                    libc::SIOCGIFINDEX => rv.index = req.ifr_ifru.ifru_ifindex,
                    libc::SIOCGIFTXQLEN => rv.qlen = req.ifr_ifru.ifru_metric,
                    libc::SIOCGIFHWADDR => rv.hwaddr = parse_mac(&req.ifr_ifru.ifru_hwaddr.sa_data),
                    _ => (),
                }
            }
        }
        rv.interface = interface.to_owned();
        rv.wireless = get_wireless(interface);
        (rv.rx, rv.tr) = getdev(interface).unwrap();
        Ok(rv)
    }
}

fn bit_to_string(bytes: f64) -> String {
    let mut index: usize = 0;
    let mut bytes = bytes;

    while bytes >= 1024.0 && index < 7 {
        bytes /= 1024.0;
        index += 1;
    }
    let units = ["bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB"];
    format!(
        "\u{1b}[0m (\u{1b}[36m {:.2} {}\u{1b}[0m )",
        bytes, units[index],
    )
}

fn style_flag(flag: i16) -> String {
    let flags = [
        "UP",
        "BROADCAST",
        "DEBUG",
        "LOOPBACK",
        "POINTOPOINT",
        "NOTRAILERS",
        "RUNNING",
        "NOARP",
        "PROMISC",
        "ALLMULTI",
        "MASTER",
        "SLAVE",
        "MULTICAST",
        "PORTSEL",
        "AUTOMEDIA",
        "DYNAMIC",
    ];

    let mut rv = String::new();
    let if_flag = flag;

    for (i, &flag_name) in flags.iter().enumerate() {
        if (if_flag & (1 << i)) != 0 {
            rv.push_str(flag_name);
            rv.push(' ');
        } else if flag_name == "UP" {
            rv.push_str("DOWN");
            rv.push(' ');
        }
    }
    rv.pop();
    rv
}

fn get_vendor(mac: String) -> Option<String> {
    use std::fs::*;
    use std::io::*;

    let file = File::open("/home/brijesh/Projects/cmac/oui").unwrap();
    let reader = BufReader::new(file);
    use hashbrown::HashMap;

    let mut v: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let d = line.unwrap();
        let d: Vec<&str> = d.split('|').collect();
        v.insert(d.get(0).unwrap().to_string(), d.get(1).unwrap().to_string());
    }
    v.get(&mac[..8]).cloned()
}

fn build_hard_info(r: &Device) -> String {
    let mut builder = Builder::new();

    builder.push_record(["MAC", "Queue", "MTU", "Flag"]);

    builder.push_record([
        r.hwaddr.to_hex_string(),
        r.qlen.to_string(),
        r.mtu.to_string(),
        format!("\x1b[32m{}\x1b[0m\n{}", r.flag, style_flag(r.flag)),
    ]);

    let vendor = get_vendor(r.hwaddr.to_hex_string());
    if let Some(v) = vendor {
        // builder.push_record(["Vendor", v.as_str()]);
        return style_table(&builder, "┐Hardware Info┌")
            .with(tabled::settings::Panel::footer(v))
            .with(Style::modern().intersection_bottom('─').intersection('*'))
            .to_string();
    }
    //.with(Modify::new(Columns::new(3..)).with(Width::wrap(30)))
    style_table(&builder, "┐Hardware Info┌").to_string()
}

fn build_ip4(r: &Device) -> String {
    let mut builder = Builder::new();

    builder.push_record(["Address", "Netmask", "Broadcast", "Destination Address"]);

    builder.push_record([
        r.addr
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
        r.netmask.to_string(),
        r.broadaddr.to_string(),
        r.dstaddr.to_string(),
    ]);
    // builder.push_record(["Public IP", ip4]);

    style_table(&builder, "┐IPv4 Info┌").to_string()
}

fn style_table(t: &Builder, head: &str) -> Table {
    let t = t.to_owned();

    t.build()
        .with(Style::modern())
        .with(Colorization::rows([
            Color::FG_BRIGHT_BLUE,
            Color::FG_BRIGHT_YELLOW | Color::BOLD,
        ]))
        .with(ColumnNames::new([head]))
        .to_owned()
    // .with(Settings::new(Alignment::center(), Alignment::center()))
}

fn build_wireless(r: &Device) -> Option<String> {
    if let Some(x) = &r.wireless {
        let mut builder = Builder::new();

        builder.push_record(["Link", "Level", "Noise"]);
        builder.push_record([x.link.to_string(), x.level.to_string(), x.noise.to_string()]);
        return Some(style_table(&builder, "┐Wireless┌").to_string());
    }
    return None;
}

fn build_ip6(r: &Vec<String>) -> String {
    let mut builder = Builder::new();

    builder.push_record([
        "Global Address",
        Ipv6Addr::from_str(r.get(0).unwrap_or(&"::".to_string()).as_str())
            .unwrap()
            .to_string()
            .as_str(),
    ]);
    builder.push_record([
        "Link-local Address",
        Ipv6Addr::from_str(r.get(1).unwrap_or(&"::".to_string()).as_str())
            .unwrap()
            .to_string()
            .as_str(),
    ]);

    builder
        .to_owned()
        .build()
        .with(Style::modern())
        .with(Colorization::columns([
            Color::FG_BLUE,
            Color::FG_BRIGHT_YELLOW,
        ]))
        .with(ColumnNames::new(["┐IPv6 Info┌"]))
        .to_string()
}

// fn build_rx(rx: Receive, all: bool) -> Builder {
//     let mut builder = Builder::new();

//     builder.push_record(["packets", rx.packets.to_string().as_str()]);
//     builder.push_record([
//         "bytes",
//         (rx.bytes.to_string() + bit_to_string(rx.bytes as f64).as_str()).as_str(),
//     ]);
//     builder.push_record(["errs", rx.errs.to_string().as_str()]);
//     builder.push_record(["drop", rx.drop.to_string().as_str()]);

//     if all == true {
//         builder.push_record(["fifo", rx.fifo.to_string().as_str()]);
//         builder.push_record(["frame", rx.frame.to_string().as_str()]);
//         builder.push_record(["compressed", rx.compressed.to_string().as_str()]);
//         builder.push_record(["multicast", rx.multicast.to_string().as_str()]);
//     }
//     builder
// }

// fn build_tx(rx: Transmit, all: bool) -> Builder {
//     let mut builder = Builder::new();

//     builder.push_record(["packets", rx.packets.to_string().as_str()]);
//     builder.push_record([
//         "bytes",
//         (rx.bytes.to_string() + bit_to_string(rx.bytes as f64).as_str()).as_str(),
//     ]);
//     builder.push_record(["errs", rx.errs.to_string().as_str()]);
//     builder.push_record(["drop", rx.drop.to_string().as_str()]);

//     if all == true {
//         builder.push_record(["fifo", rx.fifo.to_string().as_str()]);
//         builder.push_record(["colls", rx.colls.to_string().as_str()]);
//         builder.push_record(["carrier", rx.carrier.to_string().as_str()]);
//         builder.push_record(["compressed", rx.compressed.to_string().as_str()]);
//     }
//     builder
// }

// fn style_rx_tx(b: Builder, c: Color) -> String {
//     b.build()
//         .with(Style::modern())
//         .with(BorderColor::filled(c))
//         .with(Colorization::columns([
//             Color::BOLD,
//             Color::FG_BRIGHT_YELLOW,
//         ]))
//         .to_string()
// }

fn build_rx_tx(rv: &Device) -> String {
    let mut builder = Builder::new();
    builder.push_record(["", "packets", "bytes", "errs", "drop"]);

    let rx = rv.tr;
    builder.push_record([
        "\x1b[35mTransmit\x1b[0m",
        rx.packets.to_string().as_str(),
        (rx.bytes.to_string() + bit_to_string(rx.bytes as f64).as_str()).as_str(),
        rx.errs.to_string().as_str(),
        rx.drop.to_string().as_str(),
    ]);

    let rx = rv.rx;
    builder.push_record([
        "\x1b[35mReceive\x1b[0m",
        rx.packets.to_string().as_str(),
        (rx.bytes.to_string() + bit_to_string(rx.bytes as f64).as_str()).as_str(),
        rx.errs.to_string().as_str(),
        rx.drop.to_string().as_str(),
    ]);
    // builder.push_record([
    //     style_rx_tx(build_tx(rv.tra, false), Color::FG_BRIGHT_RED),
    //     style_rx_tx(build_rx(rv.rec, false), Color::FG_BRIGHT_GREEN),
    // ]);

    builder
        .build()
        .with(Colorization::rows([
            Color::FG_BLUE,
            Color::FG_BRIGHT_YELLOW,
            Color::FG_BRIGHT_YELLOW,
        ]))
        .with(Style::modern())
        .with(ColumnNames::new(["┐Statistics┌"]))
        .to_string()
}

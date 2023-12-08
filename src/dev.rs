use std::{borrow::Cow, fmt::Display};
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
use tabled::{Table, Tabled};

#[derive(Debug, Default, Clone, Copy, Tabled)]
pub struct Transmit {
    pub bytes: usize,
    pub packets: usize,
    pub errs: usize,
    pub drop: usize,
    pub fifo: usize,
    pub colls: usize,
    pub carrier: usize,
    pub compressed: usize,
}

#[derive(Debug, Default, Clone, Copy, Tabled)]
pub struct Receive {
    pub bytes: usize,
    pub packets: usize,
    pub errs: usize,
    pub drop: usize,
    pub fifo: usize,
    pub frame: usize,
    pub compressed: usize,
    pub multicast: usize,
}

#[derive(Debug, Default, Clone)]
pub struct NetworkDevice {
    pub interface: String,
    pub receive: Receive,
    pub transmit: Transmit,
}

impl Tabled for NetworkDevice {
    const LENGTH: usize = 3;

    fn fields(&self) -> Vec<Cow<'_, str>> {
        vec![
            Cow::Owned(self.interface.to_string()),
            Cow::Owned(
                Table::new([self.receive])
                    .with(tabled::settings::Style::blank())
                    .to_string(),
            ),
            Cow::Owned(
                Table::new([self.transmit])
                    .with(tabled::settings::Style::blank())
                    .to_string(),
            ),
        ]
    }

    fn headers() -> Vec<Cow<'static, str>> {
        vec![
            Cow::Borrowed("┐Interface┌"),
            Cow::Borrowed("┐Receive┌"),
            Cow::Borrowed("┐Transmit┌"),
        ]
    }
}

impl NetworkDevice {
    pub fn new(interface: String, receive: Receive, transmit: Transmit) -> NetworkDevice {
        Self {
            interface,
            receive,
            transmit,
        }
    }
}

#[inline]
pub fn get(split: &mut std::str::SplitWhitespace<'_>) -> usize {
    split.next().unwrap_or_default().parse().unwrap_or_default()
}

pub fn mai() -> Vec<NetworkDevice> {
    let contents = std::fs::read_to_string("/proc/net/dev").expect("Error reading file");
    contents
        .lines()
        .skip(2)
        .map(|line: &str| {
            let mut split: std::str::SplitWhitespace<'_> = line.split_whitespace();
            let mut interface: String = split.next().unwrap().to_string();
            interface.pop();

            NetworkDevice::new(
                interface,
                Receive {
                    bytes: get(&mut split),
                    packets: get(&mut split),
                    errs: get(&mut split),
                    drop: get(&mut split),
                    fifo: get(&mut split),
                    frame: get(&mut split),
                    compressed: get(&mut split),
                    multicast: get(&mut split),
                },
                Transmit {
                    bytes: get(&mut split),
                    packets: get(&mut split),
                    errs: get(&mut split),
                    drop: get(&mut split),
                    fifo: get(&mut split),
                    colls: get(&mut split),
                    carrier: get(&mut split),
                    compressed: get(&mut split),
                },
            )
        })
        .collect()
}

#[derive(Debug, Default, Clone, Tabled)]
pub struct WirelessInterface {
    pub interface: String,
    pub status: usize,
    pub link: f64,
    pub level: f64,
    pub noise: isize,
    pub nwid: usize,
    pub crypt: usize,
    pub frag: usize,
    pub retry: usize,
    pub misc: usize,
    pub beacon: usize,
}

pub fn parse_wireless_interfaces() -> Vec<WirelessInterface> {
    let contents = std::fs::read_to_string("/proc/net/wireless").expect("Error reading file");
    let mut interfaces = vec![];

    for line in contents.lines().skip(2) {
        let mut split: std::str::SplitWhitespace<'_> = line.split_whitespace();
        let interface = split.next().unwrap().trim_matches(':').to_string();
        // split.next();
        interfaces.push(WirelessInterface {
            interface: interface,
            status: split.next().unwrap_or_default().parse().unwrap_or_default(),
            link: split.next().unwrap_or_default().parse().unwrap_or_default(),
            level: split.next().unwrap_or_default().parse().unwrap_or_default(),
            noise: split.next().unwrap_or_default().parse().unwrap_or_default(),
            nwid: split.next().unwrap_or_default().parse().unwrap_or_default(),
            crypt: split.next().unwrap_or_default().parse().unwrap_or_default(),
            frag: split.next().unwrap_or_default().parse().unwrap_or_default(),
            retry: split.next().unwrap_or_default().parse().unwrap_or_default(),
            misc: split.next().unwrap_or_default().parse().unwrap_or_default(),
            beacon: split.next().unwrap_or_default().parse().unwrap_or_default(),
        })
    }
    interfaces
}

pub fn get_wireless(interface: &String) -> Option<WirelessInterface> {
    for i in parse_wireless_interfaces() {
        if &i.interface == interface {
            return Some(i);
        }
    }
    None
}

use libc::*;
use std::fs;
use std::io::BufReader;
use std::{collections::HashMap, io::BufRead};

use crate::change::InetModify;
use tabled::{
    builder::Builder,
    settings::{
        object::Segment,
        style::*,
        themes::{Colorization, ColumnNames},
        Alignment, Color, Modify, *,
    },
};


#[inline]
fn get_direction<'a>(flag: u16) -> &'a str {
    if flag as i32 & IFF_UP != 0 {
        return "\x1b[92m";
    }
    "\x1b[91m"
}

#[inline]
fn get_activation<'a>(flag: u16) -> &'a str {
    if flag as i32 & IFF_RUNNING != 0 {
        return "";
    }
    ""
}

#[inline]
fn get_activation_bool(flag: u16) -> bool {
    if flag as i32 & IFF_RUNNING != 0 {
        return true;
    }
    false
}


#[inline]
fn get_interface_type(flag: u16, interface: &str) -> char {
    if flag as i32 & libc::IFF_LOOPBACK != 0 {
        return '󰦛';
    }

    if std::fs::metadata(format!("/sys/class/net/{interface}/wireless")).is_ok() {
        return '';
    }
    '󰈀'
}

pub fn get_all_interface() -> Vec<String> {
    fs::read_dir("/sys/class/net/")
        .unwrap()
        .map(|x| x.unwrap().file_name().to_str().unwrap().to_string())
        .collect()
}


fn read_oui() -> HashMap<String, String>{

    let file = fs::File::open("oui").unwrap();
    let reader = BufReader::new(file);

    let mut v: HashMap<String, String> = HashMap::new();
   
    for line in reader.lines() {
        let d = line.unwrap();
        let d: Vec<&str> = d.split('|').collect();
        v.insert(d.first().unwrap().to_string(), d.get(1).unwrap().to_string());
    }
    v
}

pub fn info() {

    let mut builder = Builder::default();
    builder.set_header(["┐Interface┌", "┐MAC┌", "┐IP┌", "┐Vendor┌"]);


    let mut inet = InetModify::default();
    let v = read_oui();
    for interface in get_all_interface() {
        inet.set_interface_name(&interface);

        let flag = inet.get_flag().unwrap() as u16;

        let mac = inet.get_mac().unwrap().to_string();

        let mut ip = "-".to_string();

        if get_activation_bool(flag) {
            // ip = format!("{}\n{:?}", inet.get_ip4(), getip6(interface.as_str()))
            let ips = inet.get_ip4().unwrap();
            if ips.len() == 1 {
                ip = ips.first().unwrap().to_string();
            }
            else{
                ip =inet.get_ip4().unwrap().iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n");
                // let mut ipbuilder = Builder::default();
                // ipbuilder.push_record([ip]);
                // ip = ipbuilder.build().with(Style::rounded().remove_horizontals()).to_string();
            }
        }
 
        
        builder.push_record([
            format!(
                "{} {}{} {}",
                get_direction(flag),
                interface,
                get_activation(flag),
                get_interface_type(flag, interface.as_str()),
                
            ),
            format!("\x1b[93m{}\x1b[0m\x1b[36m{}", &mac[..8], &mac[8..]),
            ip,
            v.get(&mac[..8]).unwrap_or(&"─".to_string()).to_owned(),
        ]);
    }

    let mut table = builder.build();
    table.with(Style::rounded().remove_horizontals());

    table.with(Colorization::columns([
        Color::FG_WHITE, // no use
        Color::FG_WHITE, // no use
        Color::FG_MAGENTA,
        Color::FG_BLUE,
    ]));

    table.with(Modify::new(Segment::all()).with(Alignment::center_vertical()));
    table.with(ColumnNames::default().set_color(Color::BOLD));
    table.with(Modify::new(object::Columns::new(3..)).with(Width::wrap(25).keep_words()));

    println!("{}", table);
}

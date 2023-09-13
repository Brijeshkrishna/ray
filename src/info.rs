use std::fs::File;
use std::io::BufReader;
use std::{collections::HashMap, io::BufRead};

use tabled::{
    builder::Builder,
    settings::{style::Style, themes::Colorization, Color},
};

pub fn info() {

    let mut builder = Builder::default();
    builder.set_default_text("-");
    builder.set_header(["Interface", "MAC", "Vendor"]);

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
        let addr = std::fs::read_to_string(format!("{}/address", path.path().display()))
            .unwrap()
            .as_str()
            .trim()
            .to_string();
        let k = (addr.as_str())[0..8].to_string().to_uppercase();

        let aa = v.get(&k).unwrap_or(&"-".to_string()).to_owned();

        builder.push_record([
            path.file_name().to_str().unwrap(),
            format!(
                "\x1b[93m{}\x1b[0m\x1b[36m{}",
                k,
                (addr.as_str())[8..].to_string()
            )
            .as_str(),
            aa.as_str(),
        ]);
    }

    let mut table = builder.build();
    table.with(Style::rounded());
    table.with(Colorization::columns([
        Color::FG_BLUE,
        Color::FG_CYAN,
        Color::FG_MAGENTA,
        Color::FG_MAGENTA,
    ]));

    // table.with(Width::justify(10));
    println!("{}", table);
}

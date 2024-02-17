use std::net::Ipv4Addr;

use crate::validate::*;
use clap::{Parser, Subcommand, ValueEnum};

use clap::Args as Argss;

#[derive(Parser, Debug)]
#[command(version, about = "CLI")]
#[group( multiple = false)]
pub struct Args {
    /// interface name
    #[arg(value_parser=valid_existing_interface) ]
    pub interface: Option<String>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// change the interface propertes
    Set {
        /// interface name
        #[arg(value_parser=valid_new_interface)]
        interface: String,

        #[command(flatten)]
        c1: Change,
    },
}

#[derive(Argss, Debug)]
#[group(required = true, multiple = false)]
pub struct Change {
    /// rename the interface
    #[arg(short, long, value_name = "new interface name")]
    pub rename: Option<String>,

    /// change the MAC
    #[arg(long, value_name = "new mac addr",value_parser=is_valid_mac)]
    pub mac: Option<String>,

    /// add new IP
    #[arg(long, value_name = "new ip addr")]
    pub ip: Option<Ipv4Addr>,

    /// change destination IP
    #[arg(long)]
    pub dip: Option<Ipv4Addr>,

    /// change bordcast IP
    #[arg(short, long)]
    pub bip: Option<Ipv4Addr>,

    /// change network mask
    #[arg(long)]
    pub mask: Option<Ipv4Addr>,

    /// change MUT
    #[arg(short, long)]
    pub mtu: Option<u32>,

    /// change queue length
    #[arg(short, long)]
    pub queue: Option<u32>,

    #[arg(value_enum)]
    pub state: Option<InterfaceState>,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum InterfaceState {
    Up,
    Down,
}



use std::net::Ipv4Addr;

use crate::{change, defin, dev_info, info, validate::*};
use clap::{Parser, Subcommand, ValueEnum};

use clap::Args as Argss;

use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(version, about = "CLI")]
#[group(multiple = false)]
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

pub fn parser(args: Args) {
    match args.interface {
        Some(interface) => {
            dev_info::display(&interface);
        }
        None => match args.command {
            Some(Command::Set { interface, c1 }) => {
                let mut inet = change::InetModify::new(interface.as_str());

                c1.rename
                    .map(|new_name| inet.rename_interface(new_name.as_str()));

                c1.ip.map(|new_ip| inet.add_ip(&new_ip));
                c1.dip.map(|new_ip| inet.change_dest_ip(&new_ip));
                c1.bip.map(|new_ip| inet.change_bordcast_ip(&new_ip));
                c1.mask.map(|new_name| inet.change_netmask_ip(&new_name));

                c1.queue.map(|new_q| inet.set_queue_len(new_q));
                c1.mtu.map(|new_mtu| inet.set_mtu(new_mtu));

                c1.state.map(|x| match x {
                    InterfaceState::Up => inet.interface_up(),
                    InterfaceState::Down => inet.interface_down(),
                });

                c1.mac.map(|new_name| {
                    inet.set_mac(&defin::MacAddr::from_str(new_name.as_str()).unwrap())
                });
            }
            None => {
                info::info();
            }
        },
    }
}

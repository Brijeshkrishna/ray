use std::str::FromStr;

use clap::Parser;

mod change;
mod cli;
mod defin;
mod dev_info;
mod info;
mod validate;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    match args.interface {
        Some(interface) => {
            dev_info::display(&interface);
        }
        None => match args.command {
            Some(cli::Command::Set { interface, c1 }) => {
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
                    cli::InterfaceState::Up => inet.interface_up(),
                    cli::InterfaceState::Down => inet.interface_down(),
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

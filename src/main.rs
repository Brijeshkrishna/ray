use clap::Parser;
use cli::parser;

mod change;
mod cli;
mod defin;
mod dev_info;
mod info;
mod validate;


fn main() {

    
    let args = cli::Args::parse();
    parser(args);

}

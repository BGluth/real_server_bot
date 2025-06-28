use clap::Parser;

use crate::prog_args::{Command, ProgArgs};

mod prog_args;

fn main() {
    let p_args = ProgArgs::parse();

    match p_args.cmd {
        Command::Listen(listen_args) => {
            
        },
        Command::Query(query_args) => todo!(),
    }
}

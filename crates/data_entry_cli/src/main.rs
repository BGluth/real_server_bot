use chrono::Utc;
use clap::Parser;
use log::info;

use crate::{
    interactive::interactive_loop,
    prog_args::{EntryCommand, ProgArgs},
};

mod interactive;
mod prog_args;
mod types;

fn main() -> anyhow::Result<()> {
    if let Err(err) = run() {
        println!("Error: {}", err);
    }

    Ok(())
}

fn run() -> anyhow::Result<()> {
    let p_args = ProgArgs::parse();

    match p_args.cmd {
        EntryCommand::Interactive(interactive_args) => {
            let start_date = match interactive_args.date {
                Some(d) => d,
                None => {
                    info!("No date set! Using today date instead for set data!");
                    Utc::now() // TODO: Zero out hour...
                }
            };

            interactive_loop(start_date);
        }
        EntryCommand::PrintMonth(_) => todo!(),
    }

    Ok(())
}

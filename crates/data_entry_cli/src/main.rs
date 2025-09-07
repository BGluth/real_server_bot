use std::str::FromStr;

use anyhow::Context;
use camino::Utf8PathBuf;
use chrono::Utc;
use clap::Parser;
use log::info;
use reals_server_bot_db::db::MatchDb;

use crate::{
    interactive::interactive_loop,
    prog_args::{EntryCommand, ProgArgs},
};

mod interactive;
mod prog_args;
mod report_gen;
mod types;
mod utils;

fn main() -> anyhow::Result<()> {
    if let Err(err) = run() {
        println!("Error: {}", err);
    }

    Ok(())
}

fn run() -> anyhow::Result<()> {
    let p_args = ProgArgs::parse();
    let db_path = Utf8PathBuf::from_str(&p_args.set_db_path)
        .with_context(|| "Database root file is not a valid path")?;

    let mut db = MatchDb::open_or_crate(&db_path)?;

    match p_args.cmd {
        EntryCommand::Interactive(interactive_args) => {
            let start_date = match interactive_args.date {
                Some(d) => d,
                None => {
                    info!("No date set! Using today date instead for set data!");
                    Utc::now().naive_utc() // TODO: Zero out hour...
                }
            };

            interactive_loop(start_date, db);
        }
        EntryCommand::GetPlayerSets(_) => todo!(),
        EntryCommand::GetPlayerId(_) => todo!(),
        EntryCommand::PrintMonth(date) => {
            let date_to_use = match date.date {
                Some(d) => d,
                None => Utc::now().naive_utc(),
            };

            let all_sets_for_month = db.get_sets_for_month(date_to_use.into())?;
        }
        EntryCommand::TracePlayerMonthStats(_) => todo!(),
    }

    Ok(())
}

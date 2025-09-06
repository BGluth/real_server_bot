use std::str::FromStr;

use chrono::{DateTime, Utc};
use clap::{Args, Parser, Subcommand};
use reals_server_bot_common::types::PlayerId;

#[derive(Parser, Debug)]
pub(crate) struct ProgArgs {
    #[command(subcommand)]
    pub(crate) cmd: EntryCommand,

    #[arg(short, long, default_value = ".")]
    pub(crate) match_db_root_dir: String,
}
#[derive(Clone, Debug, Subcommand)]
pub(crate) enum EntryCommand {
    Interactive(CliDate),
    GetPlayerSets(GetPlayerSetsArgs),
    GetPlayerId(GetPlayerIdArgs),
    PrintMonth(CliDate),
    TracePlayerMonthStats(TracePlayerMonthStats),
}

#[derive(Clone, Debug)]
pub(crate) struct InteractiveArgs {
    pub(crate) start_date: Option<CliDate>,
}

#[derive(Args, Clone, Debug)]
pub(crate) struct CliDate {
    #[arg(short, long)]
    pub(crate) date: Option<DateTime<Utc>>,
}

impl FromStr for CliDate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CliDate {
            date: Some(dateparser::parse(s)?),
        })
    }
}

#[derive(Args, Clone, Debug)]
pub(crate) struct GetPlayerSetsArgs {
    pub(crate) p_id: PlayerId,
    pub(crate) num_sets: usize,
}

#[derive(Args, Clone, Debug)]
pub(crate) struct GetPlayerIdArgs {
    pub(crate) player_name: String,
}

#[derive(Args, Clone, Debug)]
pub(crate) struct TracePlayerMonthStats {
    pub(crate) p_id: PlayerId,
}

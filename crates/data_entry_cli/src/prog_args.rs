use std::str::FromStr;

use chrono::{DateTime, Utc};
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
pub(crate) struct ProgArgs {
    #[command(subcommand)]
    pub(crate) cmd: EntryCommand,
}
#[derive(Clone, Debug, Subcommand)]
pub(crate) enum EntryCommand {
    Interactive(CliDate),
    PrintMonth(CliDate),
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

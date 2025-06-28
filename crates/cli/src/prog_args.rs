use clap::{Args, Parser, Subcommand};
use reals_server_bot_common::types::{DiscordChannelId, DiscordServerId};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct ProgArgs {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    Listen(ListenArgs),
    Query(QueryArgs),
}

#[derive(Args, Debug)]
pub(crate) struct ListenArgs {
    pub server_to_listen_to: DiscordServerId,
    pub channels_to_listen_to: Vec<DiscordChannelId>,
}

#[derive(Args, Debug)]
pub(crate) struct QueryArgs {}

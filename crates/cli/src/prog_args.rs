use clap::{Args, Parser, Subcommand};
use reals_server_bot_common::types::{DiscordBotToken, DiscordChannelId, DiscordServerId};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct ProgArgs {
    #[command(subcommand)]
    pub(crate) cmd: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    Listen(ListenArgs),
    Query(QueryArgs),
}

#[derive(Args, Debug)]
pub(crate) struct ListenArgs {
    pub(crate) bot_token: DiscordBotToken,
    pub(crate) server_to_listen_to: DiscordServerId,
    pub(crate) channels_to_listen_to: Vec<DiscordChannelId>,
}

#[derive(Args, Debug)]
pub(crate) struct QueryArgs {}

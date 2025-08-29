pub mod game_msg_parser;

use reals_server_bot_common::types::{
    DiscordBotToken, DiscordChannelId, DiscordServerId, DiscordUserId,
};
use serenity::{
    Client,
    all::{Context, EventHandler, GatewayIntents, Message, Reaction, Ready},
    async_trait,
};
use tokio::runtime::Runtime;

pub struct ListenInfo {
    bot_token: DiscordBotToken,
    server_id: DiscordServerId,
    channels_to_listen_to: Vec<DiscordChannelId>,
    scan_info: Option<ScanAllMessagesBeforeTimeStamp>,
}

struct ScanAllMessagesBeforeTimeStamp {}

pub(crate) fn start_listen_on_server(info: ListenInfo) {
    let rt = Runtime::new().unwrap();

    rt.block_on(listen(info));
}

async fn listen(info: ListenInfo) {
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let client = Client::builder(info.bot_token, intents)
        .event_handler(MsgHandler)
        .await
        .expect("Failed to create handler!");
}

struct MsgHandler;

#[async_trait]
impl EventHandler for MsgHandler {
    async fn reaction_add(&self, ctx: Context, removed_reaction: Reaction) {
        todo!()
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn get_both_users_mentioned_in_msg() -> Option<(DiscordUserId, DiscordUserId)> {
    todo!()
}

fn check_if_both_parties_reacted_to_msg(ctx: Context, msg: Message) -> bool {
    todo!()
}

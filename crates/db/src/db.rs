use chrono::{DateTime, Utc};
use reals_server_bot_common::types::{DiscordUserId, GameSetData};

pub trait MatchDb {
    fn add_set(&mut self, set: PlayedSet) -> anyhow::Result<()>;
    fn add_new_player(
        &mut self,
        player_id: DiscordUserId,
        player_name: String,
    ) -> anyhow::Result<PlayerId>;
    fn add_tier(&mut self, tier_name: String) -> anyhow::Result<TierId>;
}

pub struct PlayedSet {
    game_data: GameSetData,
    date: DateTime<Utc>,
}

pub struct SetId(u64);
pub struct PlayerId(u64);
pub struct TierId(u64);

use chrono::{DateTime, Utc};
use reals_server_bot_common::types::{DiscordUserId, GameSetData};
use serde::{Deserialize, Serialize};

pub trait MatchDb {
    fn add_set(&mut self, set: PlayedSet) -> anyhow::Result<()>;

    fn add_new_player(
        &mut self,
        player_id: DiscordUserId,
        player_name: String,
        tier: TierId,
    ) -> anyhow::Result<PlayerId>;

    fn add_tier(&mut self, tier_name: String) -> anyhow::Result<TierId>;

    fn get_player_info(&self, p_id: PlayerId) -> anyhow::Result<Option<PlayerInfo>>;
}

pub struct PlayedSet {
    game_data: GameSetData,
    date: DateTime<Utc>,
}

pub struct PlayerInfo {
    name: String,
    tier: TierId,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
pub struct SetId(u64);
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
pub struct PlayerId(u64);
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
pub struct TierId(u64);

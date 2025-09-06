use std::str::FromStr;

use camino::Utf8Path;
use chrono::{DateTime, Utc};
use diesel::SqliteConnection;
use reals_server_bot_common::types::{DiscordUserId, GameSetData, TierInfo};
use serde::{Deserialize, Serialize};

pub struct PlayedSet {
    pub game_data: GameSetData,
    pub date: DateTime<Utc>,
}

pub struct PlayerInfo {
    pub name: String,
    pub tier: TierId,
    pub discord_user_id: DiscordUserId,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
pub struct SetId(u64);

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
pub struct PlayerId(u64);

impl FromStr for PlayerId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
pub struct TierId(u64);

pub struct MatchDb {
    conn: SqliteConnection,
}

impl MatchDb {
    pub fn open_or_crate(path: &Utf8Path) -> anyhow::Result<Self> {
        todo!()
    }

    pub fn add_set(&mut self, set: PlayedSet) -> anyhow::Result<()> {
        todo!()
    }

    pub fn get_sets_for_month(&self, month: SpecificMonth) -> anyhow::Result<Vec<PlayedSet>> {
        todo!()
    }

    pub fn get_set(&self, s_id: SetId) -> anyhow::Result<PlayedSet> {
        todo!()
    }

    pub fn remove_set(&self, s_id: SetId) -> anyhow::Result<()> {
        todo!()
    }

    pub fn add_new_player(
        &mut self,
        player_id: DiscordUserId,
        player_name: String,
        tier: TierId,
    ) -> anyhow::Result<PlayerId> {
        todo!()
    }

    pub fn add_tier(&mut self, t_name: String) -> anyhow::Result<TierId> {
        todo!()
    }

    pub fn get_tier(&self, t_id: TierId) -> anyhow::Result<TierInfo> {
        todo!()
    }

    pub fn add_player_info(&mut self, discord_id: DiscordUserId, p_info: PlayerInfo) {
        todo!()
    }

    pub fn get_player_tier(&self, p_id: PlayerId) -> anyhow::Result<TierId> {
        todo!()
    }

    pub fn get_player_info_for_discord_user_id(
        &self,
        u_id: DiscordUserId,
    ) -> anyhow::Result<Option<PlayerInfo>> {
        todo!()
    }
}

#[derive(Clone, Copy, Debug)]
struct SpecificMonth {}

use camino::Utf8Path;
use sled::Db;

use crate::db::MatchDb;

pub struct KvMatchDb {
    db: Db,
}

impl MatchDb for KvMatchDb {
    fn add_set(&mut self, set: crate::db::PlayedSet) -> anyhow::Result<()> {
        todo!()
    }

    fn add_new_player(
        &mut self,
        player_id: reals_server_bot_common::types::DiscordUserId,
        player_name: String,
        tier: crate::db::TierId,
    ) -> anyhow::Result<crate::db::PlayerId> {
        todo!()
    }

    fn add_tier(&mut self, tier_name: String) -> anyhow::Result<crate::db::TierId> {
        todo!()
    }

    fn get_player_info(
        &self,
        p_id: crate::db::PlayerId,
    ) -> anyhow::Result<Option<crate::db::PlayerInfo>> {
        todo!()
    }
}

impl KvMatchDb {
    fn create_or_open(path: &Utf8Path) -> anyhow::Result<Self> {
        let db = sled::open(path)?;

        Ok(Self { db })
    }
}

use camino::{Utf8Path, Utf8PathBuf};
use reals_server_bot_common::types::DiscordUserId;

use crate::db::{MatchDb, PlayedSet, PlayerId, TierId};

#[derive(Debug)]
pub struct FsMatchDb {
    db_dir: Utf8PathBuf,
}

// TODO: Replace anyhow error with actual error type at some point...
impl MatchDb for FsMatchDb {
    fn add_set(&mut self, set: PlayedSet) -> anyhow::Result<()> {
        todo!()
    }

    fn add_new_player(
        &mut self,
        player_id: DiscordUserId,
        player_name: String,
        tier_id: TierId,
    ) -> anyhow::Result<PlayerId> {
        todo!()
    }

    fn add_tier(&mut self, tier_name: String) -> anyhow::Result<TierId> {
        todo!()
    }

    fn get_player_info(&self, p_id: PlayerId) -> anyhow::Result<Option<crate::db::PlayerInfo>> {
        todo!()
    }
}

impl FsMatchDb {
    pub fn open_or_crate(db_root_dir: &Utf8Path) -> Self {
        todo!()
    }
}

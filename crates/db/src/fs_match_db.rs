use camino::Utf8PathBuf;
use reals_server_bot_common::types::DiscordUserId;

use crate::db::{MatchDb, PlayedSet, PlayerId, TierId};

#[derive(Debug)]
pub struct FsMatchDb {
    db_dir: Utf8PathBuf,
}

impl Default for FsMatchDb {
    fn default() -> Self {
        Self::new()
    }
}

impl FsMatchDb {
    pub fn new() -> Self {
        todo!()
    }
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
    ) -> anyhow::Result<PlayerId> {
        todo!()
    }

    fn add_tier(&mut self, tier_name: String) -> anyhow::Result<TierId> {
        todo!()
    }
}

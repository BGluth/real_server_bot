use chrono::{DateTime, Utc};
use reals_server_bot_common::types::GameSetData;

pub struct MatchDb {}

impl MatchDb {
    pub fn add_set(&mut self) {}
}

pub struct PlayedSet {
    game_data: GameSetData,
    date: DateTime<Utc>,
}

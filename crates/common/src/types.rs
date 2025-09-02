use derive_builder::Builder;
use serde::{Deserialize, Serialize};

pub type DiscordChannelId = u64;
pub type DiscordServerId = u64;
pub type DiscordBotToken = String;

pub type DiscordUserId = u64;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GameSetData {
    pub p1_info: PlayerInfoForSet,
    pub p2_info: PlayerInfoForSet,
    pub set_type: SetType,
}

#[derive(Builder, Clone, Deserialize, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct PlayerInfoForSet {
    #[builder(setter(into))]
    pub name: String,
    pub score: usize,

    #[builder(setter(into, strip_option))]
    pub character: Option<String>,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
pub enum SetType {
    Ft2,
    Ft3,
    Ft5,
    Ft10,
    Ftn(usize),
    Unknown,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TierInfo {}

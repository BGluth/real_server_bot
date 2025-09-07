use std::str::FromStr;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type DiscordChannelId = u64;
pub type DiscordServerId = u64;
pub type DiscordBotToken = String;

pub type DiscordUserId = u64;

pub type SetDate = chrono::NaiveDateTime;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GameSetData {
    pub p1_info: PlayerInfoForSet,
    pub p2_info: PlayerInfoForSet,
    pub set_type: SetType,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum DiscordUserIdentifier {
    Name(String),
    Id(DiscordUserId),
}

#[derive(Error, Debug)]
#[error("\"{0}\" is not a valid Discord user identifier!")]
pub struct InvalidDiscordPlayerIdentifier(String);

impl FromStr for DiscordUserIdentifier {
    type Err = InvalidDiscordPlayerIdentifier;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<DiscordUserId>()
            .map(DiscordUserIdentifier::Id)
            .unwrap_or_else(|_| DiscordUserIdentifier::Name(s.to_string())))
    }
}

#[derive(Builder, Clone, Deserialize, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct PlayerInfoForSet {
    // TODO: Also maybe support the actual player name as a string...
    #[builder(setter(custom))]
    pub user_identifier: DiscordUserIdentifier,
    pub score: usize,

    #[builder(setter(into, strip_option), default)]
    pub character: Option<String>,
}

impl PlayerInfoForSetBuilder {
    pub fn user_identifier(&mut self, v: &str) -> &mut Self {
        self.user_identifier =
            Some(DiscordUserIdentifier::from_str(v).expect("Invalid user identifier!"));
        self
    }
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
pub struct SetId(pub u32);

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
pub struct PlayerId(pub u32);

impl FromStr for PlayerId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
pub struct TierId(pub usize);

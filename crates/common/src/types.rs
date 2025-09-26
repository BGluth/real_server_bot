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
pub struct ScrappedGameSetData {
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
    pub score: u32,

    #[builder(setter(custom), default)]
    pub characters: Vec<String>,
}

impl PlayerInfoForSetBuilder {
    pub fn user_identifier(&mut self, v: &str) -> &mut Self {
        self.user_identifier =
            Some(DiscordUserIdentifier::from_str(v).expect("Invalid user identifier!"));
        self
    }

    pub fn character(&mut self, char: &str) -> &mut Self {
        match &mut self.characters {
            Some(chars) => chars.push(char.to_string()),
            None => self.characters = Some(vec![char.to_string()]),
        };

        self
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
pub enum SetType {
    Ft2,
    Ft3,
    Ft5,
    Ft10,
    Ftn(u32),
    Unknown,
}

impl SetType {
    pub fn new(left_score: u32, right_score: u32) -> Self {
        let max_score = left_score.max(right_score);

        match max_score {
            2 => SetType::Ft2,
            3 => SetType::Ft3,
            5 => SetType::Ft5,
            10 => SetType::Ft10,
            n => SetType::Ftn(n),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
pub struct SetId(pub u32);

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize)]
pub struct PlayerId(pub u32);

impl From<i32> for PlayerId {
    fn from(v: i32) -> Self {
        Self(v as u32)
    }
}

impl FromStr for PlayerId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
pub struct TierId(pub usize);

impl From<i32> for TierId {
    fn from(v: i32) -> Self {
        Self(v as usize)
    }
}

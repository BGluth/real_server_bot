use derive_builder::Builder;

pub type DiscordChannelId = u64;
pub type DiscordServerId = u64;
pub type DiscordBotToken = String;

pub type DiscordUserId = u64;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct GameSetData {
    pub p1_info: PlayerInfoForSet,
    pub p2_info: PlayerInfoForSet,
    pub set_type: SetType,
}

#[derive(Builder, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PlayerInfoForSet {
    #[builder(setter(into))]
    pub name: String,
    pub score: usize,

    #[builder(setter(into, strip_option))]
    pub character: Option<String>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SetType {
    Ft2,
    Ft3,
    Ft5,
    Ft10,
    Ftn(usize),
    Unknown,
}

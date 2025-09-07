use diesel::{Insertable, Queryable, Selectable};
use reals_server_bot_common::types::SetDate;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::players)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub tier_id: i32,
    pub discord_id: i64,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::player_aliases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PlayerAlias {
    pub player_id: i32,
    pub alias: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::sets)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Set {
    pub id: i32,
    pub left_player_id: i32,
    pub right_player_id: i32,
    pub left_score: i32,
    pub right_score: i32,
    pub date_time: SetDate,
    pub raw_input_text: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::sets)]
pub struct NewSet {
    pub left_player_id: i32,
    pub right_player_id: i32,
    pub left_score: i32,
    pub right_score: i32,
    pub date_time: SetDate,
    pub raw_input_text: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::players)]
pub struct NewPlayer {
    pub name: String,
    pub tier_id: i32,
    pub discord_id: i64,
}

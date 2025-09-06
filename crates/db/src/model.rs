use diesel::{Queryable, Selectable};
use reals_server_bot_common::types::SetDate;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::players)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Player {
    id: i32,
    name: String,
    tier_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::player_aliases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PlayerAlias {
    player_id: i32,
    alias: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::sets)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Set {
    id: i32,
    left_player_id: i32,
    right_player_id: i32,
    left_score: i32,
    right_score: i32,
    date_time: SetDate,
    raw_input_text: String,
}

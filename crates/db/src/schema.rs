// @generated automatically by Diesel CLI.

diesel::table! {
    player_aliases (player_id) {
        player_id -> Integer,
        alias -> Text,
    }
}

diesel::table! {
    players (id) {
        id -> Integer,
        name -> Text,
        tier_id -> Integer,
        discord_id -> BigInt,
    }
}

diesel::table! {
    sets (id) {
        id -> Integer,
        left_player_id -> Integer,
        right_player_id -> Integer,
        left_score -> Integer,
        right_score -> Integer,
        raw_input_text -> Text,
        date_time -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(player_aliases, players, sets,);

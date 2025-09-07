use anyhow::{Context, anyhow, bail};
use camino::Utf8Path;
use chrono::{Datelike, Days, NaiveDate, NaiveTime};
use diesel::{
    Connection, ExpressionMethods, RunQueryDsl, SelectableHelper, SqliteConnection,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use reals_server_bot_common::types::{
    DiscordUserId, GameSetData, PlayerId, SetDate, SetId, TierId,
};

use crate::{
    model::{NewPlayer, NewSet, Player, Set},
    schema::{players, sets},
};

pub struct PlayedSet {
    pub game_data: GameSetData,
    pub date: SetDate,
    pub raw_input: String,
}

pub struct MatchDb {
    conn: SqliteConnection,
}

impl MatchDb {
    pub fn open_or_crate(path: &Utf8Path) -> anyhow::Result<Self> {
        let conn = SqliteConnection::establish(path.as_ref())?;
        Ok(Self { conn })
    }

    pub fn add_set(
        &mut self,
        set: PlayedSet,
        p1_id: PlayerId,
        p2_id: PlayerId,
    ) -> anyhow::Result<()> {
        let set = NewSet {
            left_player_id: p1_id.0 as i32,
            right_player_id: p2_id.0 as i32,
            left_score: set.game_data.p1_info.score as i32,
            right_score: set.game_data.p2_info.score as i32,
            date_time: set.date,
            raw_input_text: set.raw_input,
        };

        diesel::insert_into(sets::table)
            .values(set)
            .execute(&mut self.conn)
            .with_context(|| "Error inserting set into table")?;

        Ok(())
    }

    pub fn get_sets_for_month(&mut self, spec_month: SpecificMonth) -> anyhow::Result<Vec<Set>> {
        use crate::schema::sets::dsl::*;

        let search_start_date_time = SetDate::new(
            NaiveDate::from_ymd_opt(spec_month.year, spec_month.month, 1).unwrap(),
            NaiveTime::MIN,
        );
        let num_days_in_month = Days::new(search_start_date_time.num_days_in_month() as u64);
        let search_end_date_time = search_start_date_time
            .checked_add_days(num_days_in_month)
            .unwrap();

        Ok(sets
            .filter(date_time.ge(search_start_date_time))
            .filter(date_time.le(search_end_date_time))
            .select(Set::as_select())
            .load(&mut self.conn)?)
    }

    pub fn get_set(&self, s_id: SetId) -> anyhow::Result<PlayedSet> {
        todo!()
    }

    pub fn remove_set(&mut self, s_id: SetId) -> anyhow::Result<()> {
        use crate::schema::sets::dsl::*;

        let rows_deleted = diesel::delete(sets).execute(&mut self.conn)?;
        if rows_deleted != 1 {
            bail!(format!(
                "Deleted {} rows when attempting to delete set id {}.",
                rows_deleted, s_id.0
            ))
        }

        Ok(())
    }

    pub fn add_new_player(
        &mut self,
        discord_id: DiscordUserId,
        name: String,
        tier_id: TierId,
    ) -> anyhow::Result<PlayerId> {
        let new_player = NewPlayer {
            name,
            tier_id: tier_id.0 as i32,
            discord_id: discord_id as i64,
        };

        let new_player_id = diesel::insert_into(players::table)
            .values(new_player)
            .returning(Player::as_returning())
            .get_result(&mut self.conn)?;

        Ok(PlayerId(new_player_id.id as u32))
    }

    pub fn get_player_tier(&self, p_id: PlayerId) -> anyhow::Result<TierId> {
        todo!()
    }

    pub fn get_player_info_for_discord_user_id(
        &mut self,
        u_id: DiscordUserId,
    ) -> anyhow::Result<Option<Player>> {
        use crate::schema::players::dsl::*;

        let mut player_info = players
            .filter(discord_id.eq(u_id as i64))
            .select(Player::as_select())
            .load(&mut self.conn)?;

        match player_info.len() {
            0 => Ok(None),
            1 => Ok(Some(player_info.remove(0))),
            _ => Err(anyhow!(
                "Got back more than one player with a given discord ID!"
            )),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SpecificMonth {
    pub month: u32,
    pub year: i32,
}

impl From<SetDate> for SpecificMonth {
    fn from(v: SetDate) -> Self {
        Self {
            month: v.month(),
            year: v.year(),
        }
    }
}

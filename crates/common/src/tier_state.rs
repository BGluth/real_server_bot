use std::collections::HashMap;

use camino::Utf8Path;
use serde::Deserialize;

use crate::types::{GameSetData, SetType, TierId};

#[derive(Debug, Deserialize)]
pub struct TierState {
    tier_map: [[HashMap<SetType, (i32, i32)>; 3]; 3],
}

impl TierState {
    pub fn load_from_disk_or_create(path: &Utf8Path) -> anyhow::Result<Self> {
        todo!()
    }

    pub fn calculate_points_changes_for_both_players_in_set(
        &self,
        l_tier: TierId,
        r_tier: TierId,
        set_data: &GameSetData,
    ) -> (i32, i32) {
        if set_data.p1_info.score == set_data.p2_info.score {
            panic!("Final set score can not be equal!");
        }

        let l_win_loss = set_data.p1_info.score > set_data.p2_info.score;
        self.lookup_point_change_for_players(l_tier, r_tier, set_data.set_type, l_win_loss)
    }

    fn lookup_point_change_for_players(
        &self,
        l_tier: TierId,
        r_tier: TierId,
        set_type: SetType,
        l_win_loss: bool,
    ) -> (i32, i32) {
        // Point changes in the lookup assume that the left player won.
        let (l_player_point_change, r_player_point_change) =
            self.tier_map[*l_tier][*r_tier][&set_type];

        match l_win_loss {
            false => (r_player_point_change, l_player_point_change),
            true => (l_player_point_change, r_player_point_change),
        }
    }
}

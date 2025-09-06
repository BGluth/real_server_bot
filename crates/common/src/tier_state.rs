use std::{collections::HashMap, fs, ops::Not};

use anyhow::Context;
use camino::Utf8Path;
use log::info;
use serde::{Deserialize, Serialize};

use crate::types::{GameSetData, SetType, TierId};

#[derive(Debug, Deserialize, Serialize)]
pub struct TierState {
    tier_map: [HashMap<SetType, [[i32; 2]; 3]>; 3],
}

impl Default for TierState {
    fn default() -> Self {
        Self {
            tier_map: [
                HashMap::from([
                    (SetType::Ft2, [[2, -2], [5, -1], [7, -1]]),
                    (SetType::Ft3, [[2, -2], [5, -1], [7, -1]]),
                    (SetType::Ft5, [[4, -3], [7, -2], [10, -1]]),
                    (SetType::Ft10, [[8, -4], [9, -4], [15, -2]]),
                ]),
                HashMap::from([
                    (SetType::Ft2, [[2, -4], [4, -3], [5, -2]]),
                    (SetType::Ft3, [[2, -4], [4, -3], [5, -2]]),
                    (SetType::Ft5, [[3, -6], [5, -4], [8, -3]]),
                    (SetType::Ft10, [[4, -8], [7, -5], [10, -4]]),
                ]),
                HashMap::from([
                    (SetType::Ft2, [[1, -5], [2, -3], [3, -2]]),
                    (SetType::Ft3, [[1, -5], [2, -3], [3, -2]]),
                    (SetType::Ft5, [[2, -8], [3, -5], [5, -4]]),
                    (SetType::Ft10, [[3, -10], [5, -7], [8, -5]]),
                ]),
            ],
        }
    }
}

impl TierState {
    pub fn load_from_disk_or_create(path: &Utf8Path) -> anyhow::Result<Self> {
        Ok(match fs::read(path) {
            Ok(v) => toml::from_slice(&v).with_context(|| {
                format!(
                    "Unable to deserialize \"{:?}\". It's probably corrupt.",
                    path
                )
            })?,
            Err(_) => {
                info!("No \"tier_points_map.toml\" found. Creating one using default values...");
                let v = Self::default();

                // Should never fail.
                let serialized_tier_state = toml::to_string_pretty(&v)?;

                fs::write(path, serialized_tier_state)
                    .with_context(|| format!("Failed writing \"{:?}\" to disk!.", path))?;

                v
            }
        })
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

        let l_win_loss = WinLoss::from(set_data.p1_info.score > set_data.p2_info.score);
        let r_win_loss = !l_win_loss;

        let l_pt_change =
            self.lookup_point_change_for_player(l_tier, r_tier, set_data.set_type, l_win_loss);
        let r_pt_change =
            self.lookup_point_change_for_player(r_tier, l_tier, set_data.set_type, r_win_loss);

        (l_pt_change, r_pt_change)
    }

    fn lookup_point_change_for_player(
        &self,
        p_tier: TierId,
        other_p_tier: TierId,
        set_type: SetType,
        win_loss: WinLoss,
    ) -> i32 {
        // Point changes in the lookup assume that the left player won.
        self.tier_map[p_tier.index()][&set_type][other_p_tier.index()][win_loss.index()]
    }
}

impl TierId {
    pub fn index(&self) -> usize {
        self.0 - 1
    }
}

#[derive(Clone, Copy, Debug)]
enum WinLoss {
    Win,
    Loss,
}

impl From<bool> for WinLoss {
    fn from(value: bool) -> Self {
        match value {
            false => Self::Win,
            true => Self::Loss,
        }
    }
}

impl WinLoss {
    fn index(&self) -> usize {
        match self {
            WinLoss::Win => 0,
            WinLoss::Loss => 1,
        }
    }
}

impl Not for WinLoss {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            WinLoss::Win => Self::Loss,
            WinLoss::Loss => Self::Win,
        }
    }
}

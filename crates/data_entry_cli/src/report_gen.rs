use std::{collections::HashMap, fmt::Display, iter::repeat};

use reals_server_bot_common::{
    tier_state::{TierInputSetData, TierPlayerSetInfo, TierState},
    types::PlayerId,
};
use reals_server_bot_db::{
    db::{MatchDb, SpecificMonth},
    model::{Player, Set},
};

#[derive(Debug)]
pub struct MonthlyPointsReport {
    player_points: HashMap<PlayerId, i32>,

    /// Leaderboards indexed by tier id and then player id (sorted by total points).
    tier_leaderboards: Vec<Vec<PlayerPointsForPeriod>>,
    player_map: HashMap<PlayerId, Player>,
}

impl Display for MonthlyPointsReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlayerPointsForPeriod {
    points: i32,
    player_id: PlayerId,
}

pub fn create_monthly_points_report(
    all_sets_for_month: &[Set],
    player_map: HashMap<PlayerId, Player>,
    tier_state: &TierState,
) -> MonthlyPointsReport {
    let mut player_points: HashMap<PlayerId, i32> = HashMap::new();
    let mut tier_leaderboards = Vec::from_iter(repeat(Vec::default()));

    // Sum scores for month for all players involved.
    for set in all_sets_for_month.iter() {
        let left_p_info = &player_map[&set.left_player_id.into()];
        let right_p_info = &player_map[&set.right_player_id.into()];

        let left_t_data = TierPlayerSetInfo {
            tier: left_p_info.tier_id.into(),
            score: set.left_score as u32,
        };
        let right_t_data = TierPlayerSetInfo {
            tier: right_p_info.tier_id.into(),
            score: set.right_score as u32,
        };

        let t_set_data = TierInputSetData::new(left_t_data, right_t_data);

        let (left_p_change, right_p_change) =
            tier_state.calculate_points_changes_for_both_players_in_set(&t_set_data);

        *player_points.entry(set.left_player_id.into()).or_default() += left_p_change;
        *player_points.entry(set.right_player_id.into()).or_default() += right_p_change;
    }

    // Now place them into their tiers and rank them.
    for (p_id, tot_points) in player_points.iter() {
        let p_info = &player_map[p_id];

        let p_points = PlayerPointsForPeriod {
            player_id: *p_id,
            points: *tot_points,
        };

        let leader_board = tier_leaderboards.get_mut(p_info.tier_id as usize).unwrap();
        leader_board.push(p_points);
    }

    // Finally sort them.
    for tier_leaderboard in tier_leaderboards.iter_mut() {
        tier_leaderboard.sort();
    }

    MonthlyPointsReport {
        player_points,
        tier_leaderboards,
        player_map,
    }
}

#[derive(Debug)]
pub struct PlayerMonthlyScoreTrace {}

impl Display for PlayerMonthlyScoreTrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub fn create_player_monthly_score_trace(
    db: &MatchDb,
    month: SpecificMonth,
    player_id: PlayerId,
) -> PlayerMonthlyScoreTrace {
    todo!()
}

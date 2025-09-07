use reals_server_bot_common::types::PlayerId;
use reals_server_bot_db::{
    db::{MatchDb, SpecificMonth},
    model::Set,
};

#[derive(Debug)]
pub struct MonthlyPointsReport {}

pub fn create_monthly_points_report(all_sets_for_month: &[Set]) -> MonthlyPointsReport {
    todo!()
}

#[derive(Debug)]
pub struct PlayerMonthlyScoreTrace {}

pub fn create_player_monthly_score_trace(
    db: &MatchDb,
    month: SpecificMonth,
    player_id: PlayerId,
) -> PlayerMonthlyScoreTrace {
    todo!()
}

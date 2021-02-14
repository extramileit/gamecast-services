use std::str::FromStr;
use crate::billiards::player::Player;
use crate::billiards::team::Team;
use chrono::{Local, DateTime};

#[derive(Debug)]
pub enum Action {
    BeginTeamCompetition { home_team_id: i32, away_team_id: i32, event_date: DateTime<Local>, venue_id: i32 },
    BeginMatch { team_competition_id: i32, top_player_id: i32, bottom_player_id: i32 },
    BeginGame { match_id: i32, game_index: u8, breaker_player_id: i32 },
    TableRun { match_id: i32, game_index: u8 },
    EightBallOnBreak{ match_id: i32, game_index: u8 },
    Timeout{ match_id: i32, game_index: u8, player_id: i32 },
    GameComplete { match_id: i32, game_index: u8, winning_player_id: i32, winning_score: u32, losing_score: u32},
    InningComplete { match_id: i32, game_index: u8},
}

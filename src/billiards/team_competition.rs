use crate::billiards::team::Team;
// use chrono::{DateTime, Local};
use crate::billiards::individual_match::Match;
use crate::billiards::venue::Venue;

pub struct TeamCompetition {
    team_competition_id: i32,
    home_team: Team,
    away_team: Team,
    // start_date: DateTime<Local>,
    venue: Venue,
    matches: Vec<Match>,
}

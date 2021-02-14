use crate::billiards::player::Player;

pub struct Team {
    team_id: i32,
    team_name: String,
    division: Division,
    players: Vec<Player>,
}

pub enum Division {
    Leisure,
    Advanced,
    Masters,
}

use crate::billiards::player::Player;

pub struct Game {
    winning_score: u32,
    losing_score: u32,
    inning_count: u32,
    breaking_player: Player,
}

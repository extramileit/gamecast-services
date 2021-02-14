use crate::billiards::game::Game;
use crate::billiards::player::Player;

pub struct Match {
    top: Player,
    bottom: Player,
    games: Vec<Game>,
    winning_score: u32,
    losing_score: u32
}

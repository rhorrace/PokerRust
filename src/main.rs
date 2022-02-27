mod calc;
mod card;
mod game;

use crate::game::Game;

fn main() {
    let mut games: Game = Game::new();
    games.play_seven_card_stud();
}
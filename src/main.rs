mod card;
mod calc;
mod game;

use crate::game::Game;

fn main() {
    let mut games: Game = Game::new();
    let choice = 1;
    match choice {
        1 => games.play_five_card_draw(),
        _ => games.play_seven_card_stud()
    }
}

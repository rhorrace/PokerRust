mod calc;
mod card;
mod game;

use crate::game::{Game, read_user};

fn main() {
    let mut games: Game = Game::new();
    loop {
        let choice: i8 = read_user("1) 7 Card Stud 2) Texas Hold'em 0) Quit: ");
        if choice == 0 {
            break;
        }
        match choice {
            1 => games.play_seven_card_stud(),
            2 => games.play_texas_holdem(),
            _ => {}
        }
    }
    println!("Goodbye!");
}
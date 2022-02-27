extern crate rand;

use crate::calc::{calc_rank, Rank, calc_best_hand};
use crate::card::{Card, Name, Name::*, Suit, Suit::*};
use rand::{seq::SliceRandom, thread_rng};
use std::collections::VecDeque;
use std::io;
use std::io::Write;
use crate::game::Phase::*;

pub fn read_user(output: &str) -> i8 {
    print!("{}", output);
    io::stdout().flush().expect("Error: Flush failed");
    let mut input = String::new();
    let mut response = -1;
    io::stdin().read_line(&mut input).expect("Error: Read failed");
    if let Ok(num) = input.trim().parse::<i8>() {
        response = num;
    }
    response
}

pub enum Phase {
    Clean,
    DealStud,
    DealTexas,
    Flop,
    Turn,
    River,
    Winner,
}

pub struct Game {
    deck: VecDeque<Card>,
    player: Vec<Card>,
    computer: Vec<Card>,
    community: Vec<Card>,
    //burned: Vec<Card>
}

impl Game {

    // Constructor(s)

    pub fn new() -> Game {
        Game {
            deck: build_deck(),
            player: Vec::new(),
            computer: Vec::new(),
            community: Vec::new(),
            //burned: Vec::new()
        }
    }

    // Public functions

    pub fn play_seven_card_stud(&mut self) {
        let mut choice: i8;
        loop {
            // Clear table if needed
            self.update_game(Clean);

            // Deal
            self.update_game(DealStud);
            choice = read_user("Enter) Continue 1) Quit: ");
            if choice == 1 {
                break;
            }

            // Winner
            self.update_game(Winner);
            choice = read_user("Enter) Continue 1) Quit: ");
            if choice == 1 {
                break;
            }
        }
    }

    pub fn play_texas_holdem(&mut self) {
        let mut choice: i8;
        loop {
            // Clear table if needed
            self.update_game(Clean);

            // Deal
            self.update_game(DealTexas);
            choice = read_user("Enter) Continue 1) Quit: ");
            if choice == 1 {
                break;
            }

            // Flop
            self.update_game(Flop);
            choice = read_user("Enter) Continue 1) Quit: ");
            if choice == 1 {
                break;
            }

            // Turn
            self.update_game(Turn);
            choice = read_user("Enter) Continue 1) Quit: ");
            if choice == 1 {
                break;
            }

            // River
            self.update_game(River);
            choice = read_user("Enter) Continue 1) Quit: ");
            if choice == 1 {
                break;
            }

            // Winner
            self.update_game(Winner);
            choice = read_user("Enter) Continue 1) Quit: ");
            if choice == 1 {
                break;
            }
        }
    }

    // Private functions

    fn clean(&mut self) {
        if !self.community.is_empty() {
            self.deck.extend(&self.community);
            self.community.clear();
        }
        if !self.player.is_empty() {
            self.deck.extend(&self.player);
            self.player.clear();
        }
        if !self.computer.is_empty() {
            self.deck.extend(&self.computer);
            self.computer.clear();
        }
    }

    fn deal(&mut self, n: usize) {
        let cards: Vec<Card> = self.deck.drain(..(n * 2)).collect();
        for pair in cards.chunks(2) {
            self.player.push(pair[0]);
            self.computer.push(pair[1]);
        }
    }

    fn display_table(&self, show_computer: bool) {
        println!("Player:   {:?}", self.player);

        if show_computer {
            println!("Computer: {:?}", self.computer);
        }

        println!("Community: {:?}", self.community);
    }

    fn shuffle(&mut self) {
        let mut deck_vec: Vec<Card> = self.deck.iter()
            .map(|&c| c)
            .collect();
        deck_vec.shuffle(&mut thread_rng());
        self.deck = VecDeque::from(deck_vec);
    }

    fn update_community(&mut self, n: usize) {
        let cards: Vec<Card> = self.deck.drain(..n).collect();
        self.community.extend(cards);
    }

    fn update_game(&mut self, phase: Phase) {
        print!("\x1B[2J\x1B[1;1H");
        match phase {
            Clean => {
                self.clean();
                self.shuffle();
            }
            DealStud => {
                self.deal(7);
                self.display_table(false);
            }
            DealTexas => {
                self.deal(2);
                self.display_table(false);
            }
            Flop => {
                self.update_community(3);
                self.display_table(false);
            }
            Turn | River => {
                self.update_community(1);
                self.display_table(false);
            }
            Winner => {
                self.display_table(true);
                self.winner();
            }
        }
    }

    fn winner(&self) {
        let mut player_hand = self.player.clone();
        let mut computer_hand = self.computer.clone();

        player_hand.extend(&self.community);
        computer_hand.extend(&self.community);

        let player_rank: Rank = calc_rank(&player_hand);
        let computer_rank: Rank = calc_rank(&computer_hand);

        let player_best_hand: Vec<Card> = calc_best_hand(&player_hand, player_rank);
        let computer_best_hand: Vec<Card> = calc_best_hand(&computer_hand, computer_rank);

        if player_rank > computer_rank {
            println!("You Win!");
        } else if player_rank < computer_rank {
            println!("You Lose!");
        } else {
            self.tie_breaker(&player_best_hand, &computer_best_hand);
        }

        println!("Player:   {}", player_rank);
        println!("\t{:?}", player_best_hand);
        println!("Computer: {}", computer_rank);
        println!("\t{:?}", computer_best_hand);
    }

    fn tie_breaker(&self, player: &[Card], computer: &[Card]) {
        for (p, c) in player.iter().zip(computer) {
            if p > c {
                println!("You Win!");
                return;
            } else if p < c {
                println!("You Lose!");
                return;
            }
        }
        println!("It's a Tie!");
    }
}

fn build_deck() -> VecDeque<Card> {
    let names: [Name; 13] = [Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, AceHigh];
    let suits: [Suit; 4] = [Hearts, Diamonds, Spades, Clubs];
    let mut deck: VecDeque<Card> = VecDeque::new();
    suits.iter()
        .for_each(|&suit| {
            names.iter()
                .for_each(|&name| {
                    deck.push_back(Card(name, suit));
                });
        });
    deck
}
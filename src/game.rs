extern crate rand;

use crate::calc::{calc_rank, Rank};
use crate::card::Card;
use rand::{seq::SliceRandom, thread_rng};
use std::collections::VecDeque;

pub struct Game {
    deck: VecDeque<Card>,
    player: Vec<Card>,
    computer: Vec<Card>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            deck: build_deck(),
            player: Vec::new(),
            computer: Vec::new(),
        }
    }

    pub fn play_seven_card_stud(&mut self) {
        self.shuffle();
        self.deal(7);
        self.display_hands();
        self.winner();
    }

    fn deal(&mut self, n: u8) {
        for _ in 0..n {
            self.player.push(self.deck.pop_front().unwrap());
            self.computer.push(self.deck.pop_front().unwrap());
        }
    }

    fn display_hands(&self) {
        println!("Computer: {:?}", self.computer);
        println!("Player:   {:?}", self.player);
    }

    fn shuffle(&mut self) {
        let mut deck_vec: Vec<Card> = self.deck.iter()
            .map(|&c| c)
            .collect();
        deck_vec.shuffle(&mut thread_rng());
        self.deck = VecDeque::from(deck_vec);
    }

    fn winner(&self) {
        let player_rank: Rank = calc_rank(&self.player);
        let computer_rank: Rank = calc_rank(&self.computer);
        if player_rank > computer_rank {
            println!("You Win!");
        } else if player_rank < computer_rank {
            println!("You Lose!");
        } else {
            println!("It's a Tie!")
        }
    }
}

fn build_deck() -> VecDeque<Card> {
    let mut deck: VecDeque<Card> = VecDeque::new();
    "HDSC".chars()
        .for_each(|suit| {
            (2..=14).for_each(|i| {
                let name: char = match i {
                    2 => '2',
                    3 => '3',
                    4 => '4',
                    5 => '5',
                    6 => '6',
                    7 => '7',
                    8 => '8',
                    9 => '9',
                    11 => 'J',
                    12 => 'Q',
                    13 => 'K',
                    14 => 'A',
                    _ => '_',
                };
                deck.push_back(Card(name, suit, i));
            });
        });
    deck
}
extern crate rand;

use crate::calc::{calc_rank, Rank, calc_best_hand};
use crate::card::{Card, Name, Name::*, Suit, Suit::*};
use rand::{seq::SliceRandom, thread_rng};
use std::collections::VecDeque;

pub struct Game {
    deck: VecDeque<Card>,
    player: Vec<Card>,
    computer: Vec<Card>,
    //burned: Vec<Card>
}

impl Game {

    // Constructor(s)

    pub fn new() -> Game {
        Game {
            deck: build_deck(),
            player: Vec::new(),
            computer: Vec::new(),
            //burned: Vec::new()
        }
    }

    // Public functions

    pub fn play_seven_card_stud(&mut self) {
        self.shuffle();
        self.deal(7);
        self.display_hands();
        self.winner();
    }

    pub fn play_texas_holdem(&mut self) {

    }

    // Private functions

    fn deal(&mut self, n: u8) {
        for _ in 0..n {
            self.player.push(self.deck.pop_front().unwrap());
            self.computer.push(self.deck.pop_front().unwrap());
        }
    }

    fn display_hands(&self) {
        println!("Player:   {:?}", self.player);
        println!("Computer: {:?}", self.computer);
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
        let player_best_hand: Vec<Card> = calc_best_hand(&self.player, player_rank);
        let computer_best_hand: Vec<Card> = calc_best_hand(&self.computer, computer_rank);
        println!("Player:   {}", player_rank);
        println!("\t{:?}", player_best_hand);
        println!("Computer: {}", computer_rank);
        println!("\t{:?}", computer_best_hand);
        if player_rank > computer_rank {
            println!("You Win!");
        } else if player_rank < computer_rank {
            println!("You Lose!");
        } else {
            self.tie_breaker(&player_best_hand, &computer_best_hand);
        }
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
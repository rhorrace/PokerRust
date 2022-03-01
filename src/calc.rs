use crate::calc::Rank::*;
use crate::card::{Card, Name, Name::*, Suit};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    Straight,
    Flush,
    FullHouse,
    FourOfKind,
    StraightFlush,
    RoyalFlush,
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let rank_str = match self {
            HighCard => "High Card",
            OnePair => "One Pair",
            TwoPair => "Two Pair",
            ThreeOfKind => "Three of a Kind",
            Straight => "Straight",
            Flush => "Flush",
            FullHouse => "Full House",
            FourOfKind => "Four of a Kind",
            StraightFlush => "Straight",
            RoyalFlush => "Royal Flush"
        };
        write!(f, "{}", rank_str)
    }
}

// Calculates the best hand according to the calculated rank
pub fn calc_best_hand(hand: &[Card], rank: Rank) -> Vec<Card> {
    let mut cards: Vec<Card> = hand.to_vec();
    let mut value_count: HashMap<Name, u8> = HashMap::new();
    hand.iter()
        .for_each(|card| {
            *value_count.entry(card.0).or_insert(0) += 1;
        });

    // Sort cards according to rank
    match rank {
        HighCard | Flush | StraightFlush | RoyalFlush => {
            cards.sort_by(|a, b| b.cmp(a));
        }
        OnePair | TwoPair | ThreeOfKind | FullHouse | FourOfKind => {
            cards.sort_by(|a, b| value_count[&b.0].cmp(&value_count[&a.0])
                .then_with(|| b.cmp(a))
                .then_with(|| a.1.cmp(&b.1)));
        }
        Straight => {
            cards.sort_by(|a, b| a.cmp(b)
                .then_with(|| a.1.cmp(&b.1)));
            cards.dedup();
            cards.reverse();
        }
    }

    // Get most frequent suit if hand is a type of Flush
    if rank == Flush || rank == StraightFlush || rank == RoyalFlush {
        let max_suit: Suit = mode_suit(&cards).unwrap();
        cards = cards.iter()
            .filter(|c| c.1 == max_suit)
            .map(|&c| c)
            .collect();
    }

    // Get best hand depending on rank
    match rank {
        HighCard | OnePair | ThreeOfKind | Flush | RoyalFlush =>
            cards.iter()
                .take(5)
                .map(|&c| c)
                .collect(),
        TwoPair | FourOfKind => {
            let mut best_hand: Vec<Card> = cards[0..4].to_vec();
            let mut kicker: Vec<Card> = cards.iter()
                .skip(4)
                .map(|&c| c)
                .collect();
            kicker.sort_by(|a, b| b.cmp(a));
            if !kicker.is_empty() {
                best_hand.push(kicker[0]);
            }
            best_hand
        }
        Straight | StraightFlush => {
            if cards[0].0 == AceHigh {
                let ace_low: Card = Card(AceLow, cards[0].1);
                cards.push(ace_low);
            }
            cards.windows(5)
                .filter(|straight| straight[0].0 as u8 - straight[4].0 as u8 == 4)
                .take(1)
                .flatten()
                .map(|&c| c)
                .collect()
        }
        FullHouse => {
            let mut best_hand: Vec<Card> = cards[0..3].to_vec();
            best_hand.sort_by(|a, b| a.1.cmp(&b.1));
            let mut pairs: Vec<Card> = cards.iter()
                .skip(3)
                .filter(|&c| value_count[&c.0] >= 2)
                .map(|&c| c)
                .collect();
            pairs.sort_by(|a, b| b.cmp(a)
                .then_with(|| a.1.cmp(&b.1)));
            let mut pair: Vec<Card> = pairs[0..2].to_vec();
            best_hand.append(&mut pair);
            best_hand
        }
    }
}

pub fn calc_rank(hand: &[Card]) -> Rank {
    if hand.len() < 5 {
        return other_rank(&hand);
    }

    let mut cards: Vec<Card> = hand.to_vec();
    cards.sort_by(|a, b| a.cmp(b));
    let mut uniq_cards: Vec<Card> = cards.clone();
    cards.reverse();
    uniq_cards.dedup();
    uniq_cards.reverse();

    if let Some(rank) = check_flush(&cards) {
        rank
    } else if is_straight(&uniq_cards) {
        Straight
    } else {
        other_rank(&cards)
    }
}

fn check_flush(hand: &[Card]) -> Option<Rank> {
    let some_suit: Option<Suit> = mode_suit(&hand);
    if some_suit == None {
        return None;
    }

    let suit: Suit = some_suit.unwrap();
    let flush: Vec<Card> = hand.iter()
        .filter(|&c| c.1 == suit)
        .map(|&c| c)
        .collect();
    if flush.len() < 5 {
        None
    } else if !is_straight(&flush) {
        Some(Flush)
    } else if flush[0].0 == AceHigh && flush[4].0 == Ten {
        Some(RoyalFlush)
    } else {
        Some(StraightFlush)
    }
}

fn is_straight(hand: &[Card]) -> bool {
    if hand.len() < 5 {
        return false;
    }

    let mut cards: Vec<Card> = hand.to_vec();

    if cards[0].0 == AceHigh {
        let ace_low: Card = Card(AceLow, cards[0].1);
        cards.push(ace_low);
    }

    cards.windows(5)
        .any(|straight| straight[0].0 as u8 - straight[4].0 as u8 == 4)
}

fn mode_suit(cards: &[Card]) -> Option<Suit> {
    let mut suit_count: HashMap<Suit, u8> = HashMap::new();

    cards.iter()
        .map(|card| card.1)
        .max_by_key(|&suit| {
            let count = suit_count.entry(suit).or_insert(0);
            *count += 1;
            *count
        })
}

fn other_rank(hand: &[Card]) -> Rank {
    let mut value_count: HashMap<Name, u8> = HashMap::new();
    let mut counts: HashMap<u8, u8> = (1..=4).map(|i| (i, 0)).collect::<HashMap<_, _>>();
    hand.iter()
        .for_each(|card| *value_count.entry(card.0).or_insert(0) += 1);
    value_count.values()
        .for_each(|&i| *counts.entry(i).or_insert(0) += 1);
    if counts[&4] >= 1 {
        FourOfKind
    } else if counts[&3] >= 1 {
        match counts[&3] > 1 || counts[&2] >= 1 {
            true => FullHouse,
            false => ThreeOfKind
        }
    } else if counts[&2] >= 2 {
        TwoPair
    } else if counts[&2] == 1 {
        OnePair
    } else {
        HighCard
    }
}

#[cfg(test)]
mod calc_tests {
    use crate::calc::{calc_rank, Rank::*, calc_best_hand};
    use crate::card::{Card, Name::*, Suit::*};

    #[test]
    fn test_rank() {
        assert!(RoyalFlush > Straight);
        assert_eq!(Flush, Flush);
        assert!(HighCard < OnePair);
    }

    #[test]
    fn test_royal_flush() {
        let hand: Vec<Card> = vec![Card(AceHigh, Hearts),
                                   Card(King, Hearts),
                                   Card(Two, Diamonds),
                                   Card(Queen, Hearts),
                                   Card(Seven, Spades),
                                   Card(Jack, Hearts),
                                   Card(Ten, Hearts)];
        assert_eq!(calc_rank(&hand), RoyalFlush);
        assert_eq!(calc_best_hand(&hand, RoyalFlush), vec![Card(AceHigh, Hearts),
                                                           Card(King, Hearts),
                                                           Card(Queen, Hearts),
                                                           Card(Jack, Hearts),
                                                           Card(Ten, Hearts)]);
    }

    #[test]
    fn test_straight_flush() {
        let high_straight: Vec<Card> = vec![Card(Nine, Hearts),
                                            Card(King, Hearts),
                                            Card(Two, Diamonds),
                                            Card(Queen, Hearts),
                                            Card(Seven, Spades),
                                            Card(Jack, Hearts),
                                            Card(Ten, Hearts)];
        let low_straight: Vec<Card> = vec![Card(AceHigh, Hearts),
                                           Card(Three, Hearts),
                                           Card(Two, Hearts),
                                           Card(Queen, Diamonds),
                                           Card(Five, Hearts),
                                           Card(Jack, Spades),
                                           Card(Four, Hearts)];
        assert_eq!(calc_rank(&high_straight), StraightFlush);
        assert_eq!(calc_best_hand(&high_straight, StraightFlush), vec![Card(King, Hearts),
                                                                       Card(Queen, Hearts),
                                                                       Card(Jack, Hearts),
                                                                       Card(Ten, Hearts),
                                                                       Card(Nine, Hearts)]);
        assert_eq!(calc_rank(&low_straight), StraightFlush);
        assert_eq!(calc_best_hand(&low_straight, StraightFlush), vec![Card(Five, Hearts),
                                                                      Card(Four, Hearts),
                                                                      Card(Three, Hearts),
                                                                      Card(Two, Hearts),
                                                                      Card(AceLow, Hearts)]);
    }

    #[test]
    fn test_four_of_kind() {
        let hand: Vec<Card> = vec![Card(Nine, Hearts),
                                   Card(Two, Diamonds),
                                   Card(Nine, Spades),
                                   Card(Nine, Diamonds),
                                   Card(Two, Hearts),
                                   Card(Nine, Clubs),
                                   Card(Two, Clubs)];
        let hand2: Vec<Card> = vec![Card(Nine, Hearts),
                                    Card(Nine, Spades),
                                    Card(Nine, Diamonds),
                                    Card(Nine, Clubs)];
        assert_eq!(calc_rank(&hand), FourOfKind);
        assert_eq!(calc_best_hand(&hand, FourOfKind), vec![Card(Nine, Hearts),
                                                           Card(Nine, Diamonds),
                                                           Card(Nine, Spades),
                                                           Card(Nine, Clubs),
                                                           Card(Two, Hearts)]);
        assert_eq!(calc_rank(&hand2), FourOfKind);
        assert_eq!(calc_best_hand(&hand2, FourOfKind), vec![Card(Nine, Hearts),
                                                            Card(Nine, Diamonds),
                                                            Card(Nine, Spades),
                                                            Card(Nine, Clubs)]);
    }

    #[test]
    fn test_full_house() {
        let hand1: Vec<Card> = vec![Card(Nine, Hearts),
                                    Card(Two, Diamonds),
                                    Card(Nine, Spades),
                                    Card(Nine, Diamonds),
                                    Card(Two, Hearts),
                                    Card(Three, Clubs),
                                    Card(Two, Clubs)];
        let hand2: Vec<Card> = vec![Card(Three, Hearts),
                                    Card(Two, Diamonds),
                                    Card(Nine, Spades),
                                    Card(Nine, Diamonds),
                                    Card(Two, Hearts),
                                    Card(Three, Clubs),
                                    Card(Two, Clubs)];
        assert_eq!(calc_rank(&hand1), FullHouse);
        assert_eq!(calc_best_hand(&hand1, FullHouse), vec![Card(Nine, Hearts),
                                                           Card(Nine, Diamonds),
                                                           Card(Nine, Spades),
                                                           Card(Two, Hearts),
                                                           Card(Two, Diamonds)]);
        assert_eq!(calc_rank(&hand2), FullHouse);
        assert_eq!(calc_best_hand(&hand2, FullHouse), vec![Card(Two, Hearts),
                                                           Card(Two, Diamonds),
                                                           Card(Two, Clubs),
                                                           Card(Nine, Diamonds),
                                                           Card(Nine, Spades)]);
    }

    #[test]
    fn test_flush() {
        let hand: Vec<Card> = vec![Card(Nine, Hearts),
                                   Card(King, Hearts),
                                   Card(Two, Diamonds),
                                   Card(Two, Hearts),
                                   Card(Seven, Spades),
                                   Card(Jack, Hearts),
                                   Card(Ten, Hearts)];
        assert_eq!(calc_rank(&hand), Flush);
        assert_eq!(calc_best_hand(&hand, Flush), vec![Card(King, Hearts),
                                                      Card(Jack, Hearts),
                                                      Card(Ten, Hearts),
                                                      Card(Nine, Hearts),
                                                      Card(Two, Hearts)]);
    }

    #[test]
    fn test_straight() {
        let high_straight: Vec<Card> = vec![Card(Nine, Diamonds),
                                            Card(King, Clubs),
                                            Card(Two, Diamonds),
                                            Card(Queen, Hearts),
                                            Card(Seven, Spades),
                                            Card(Jack, Hearts),
                                            Card(Ten, Hearts)];
        let low_straight: Vec<Card> = vec![Card(Four, Diamonds),
                                           Card(AceHigh, Clubs),
                                           Card(Two, Diamonds),
                                           Card(Queen, Hearts),
                                           Card(Five, Spades),
                                           Card(Jack, Hearts),
                                           Card(Three, Hearts)];
        assert_eq!(calc_rank(&high_straight), Straight);
        assert_eq!(calc_best_hand(&high_straight, Straight), vec![Card(King, Clubs),
                                                                  Card(Queen, Hearts),
                                                                  Card(Jack, Hearts),
                                                                  Card(Ten, Hearts),
                                                                  Card(Nine, Diamonds)]);
        assert_eq!(calc_rank(&low_straight), Straight);
        assert_eq!(calc_best_hand(&low_straight, Straight), vec![Card(Five, Spades),
                                                                 Card(Four, Diamonds),
                                                                 Card(Three, Hearts),
                                                                 Card(Two, Diamonds),
                                                                 Card(AceLow, Clubs)]);
    }

    #[test]
    fn test_three_of_kind() {
        let hand: Vec<Card> = vec![Card(Nine, Hearts),
                                   Card(Seven, Diamonds),
                                   Card(Three, Spades),
                                   Card(Nine, Diamonds),
                                   Card(Two, Hearts),
                                   Card(Nine, Clubs),
                                   Card(Five, Clubs)];
        assert_eq!(calc_rank(&hand), ThreeOfKind);
        assert_eq!(calc_best_hand(&hand, ThreeOfKind), vec![Card(Nine, Hearts),
                                                            Card(Nine, Diamonds),
                                                            Card(Nine, Clubs),
                                                            Card(Seven, Diamonds),
                                                            Card(Five, Clubs)]);
    }

    #[test]
    fn test_two_pair() {
        let hand: Vec<Card> = vec![Card(Nine, Hearts),
                                   Card(Two, Diamonds),
                                   Card(Nine, Spades),
                                   Card(Eight, Diamonds),
                                   Card(Two, Hearts),
                                   Card(Six, Clubs),
                                   Card(Four, Clubs)];
        let hand2: Vec<Card> = vec![Card(Nine, Hearts),
                                    Card(Two, Diamonds),
                                    Card(Nine, Spades),
                                    Card(Two, Hearts), ];
        assert_eq!(calc_rank(&hand), TwoPair);
        assert_eq!(calc_best_hand(&hand, TwoPair), vec![Card(Nine, Hearts),
                                                        Card(Nine, Spades),
                                                        Card(Two, Hearts),
                                                        Card(Two, Diamonds),
                                                        Card(Eight, Diamonds)]);
        assert_eq!(calc_rank(&hand2), TwoPair);
        assert_eq!(calc_best_hand(&hand2, TwoPair), vec![Card(Nine, Hearts),
                                                         Card(Nine, Spades),
                                                         Card(Two, Hearts),
                                                         Card(Two, Diamonds)]);
    }

    #[test]
    fn test_one_pair() {
        let hand: Vec<Card> = vec![Card(AceHigh, Hearts),
                                   Card(Two, Diamonds),
                                   Card(Nine, Spades),
                                   Card(Eight, Diamonds),
                                   Card(Two, Hearts),
                                   Card(Six, Clubs),
                                   Card(Four, Clubs)];
        assert_eq!(calc_rank(&hand), OnePair);
        assert_eq!(calc_best_hand(&hand, OnePair), vec![Card(Two, Hearts),
                                                        Card(Two, Diamonds),
                                                        Card(AceHigh, Hearts),
                                                        Card(Nine, Spades),
                                                        Card(Eight, Diamonds)]);
    }

    #[test]
    fn test_high_card() {
        let hand: Vec<Card> = vec![Card(AceHigh, Hearts),
                                   Card(Jack, Diamonds),
                                   Card(Nine, Spades),
                                   Card(Eight, Diamonds),
                                   Card(Two, Hearts),
                                   Card(Six, Clubs),
                                   Card(Four, Clubs)];
        assert_eq!(calc_rank(&hand), HighCard);
        assert_eq!(calc_best_hand(&hand, HighCard), vec![Card(AceHigh, Hearts),
                                                         Card(Jack, Diamonds),
                                                         Card(Nine, Spades),
                                                         Card(Eight, Diamonds),
                                                         Card(Six, Clubs)]);
    }
}
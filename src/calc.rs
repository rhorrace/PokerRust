use crate::card::Card;
use crate::calc::Rank::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, PartialOrd)]
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

pub fn calc_rank(hand: &[Card]) -> Rank {
    let mut cards: Vec<Card> = hand.to_vec();
    cards.sort();
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
    if hand.len() < 5 {
        None
    } else {
        let mut groups: HashMap<char, Vec<Card>> = "HDSC".chars().map(|c| (c, Vec::new())).collect();
        for card in hand {
            groups.entry(card.1)
                .or_insert(Vec::new())
                .push(*card);
        }
        let mut flush: Vec<Card> = Vec::new();
        for cards in groups.values() {
            if cards.len() >= 5 {
                flush = cards.clone();
                break;
            }
        }
        if flush.is_empty() {
            None
        } else if is_straight(&flush) {
            if flush[0].0 == 'A' && flush[4].2 == 10 {
                Some(RoyalFlush)
            } else {
                Some(StraightFlush)
            }
        } else {
            Some(Flush)
        }
    }
}

fn is_straight(hand: &[Card]) -> bool {
    if hand.len() < 5 {
        false
    } else {
        let mut cards = hand.to_vec();
        if cards[0].0 == 'A' {
            let ace_low: Card = Card('A', cards[0].1, 1);
            cards.push(ace_low);
        }
        cards.windows(5)
            .any(|straight| straight[0].2 - straight[4].2 == 4)
    }
}

fn other_rank(hand: &[Card]) -> Rank {
    let mut value_count: HashMap<u8, u8> = HashMap::new();
    let mut counts: HashMap<u8, u8> = (1..=4).map(|i| (i, 0)).collect::<HashMap<_, _>>();
    hand.iter()
        .for_each(|card| *value_count.entry(card.2).or_insert(0) += 1);
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
    use crate::card::Card;
    use crate::calc::{calc_rank, Rank::*};

    #[test]
    fn test_rank() {
        assert!(RoyalFlush > Straight);
        assert_eq!(Flush, Flush);
        assert!(HighCard < OnePair);
    }

    #[test]
    fn test_royal_flush() {
        let hand: Vec<Card> = vec![Card('A', 'H', 14), Card('K', 'H', 13), Card('2', 'D', 2), Card('Q', 'H', 12), Card('7', 'S', 7), Card('J', 'H', 11), Card('_', 'H', 10)];
        assert_eq!(calc_rank(&hand), RoyalFlush)
    }

    #[test]
    fn test_straight_flush() {
        let high_straight: Vec<Card> = vec![Card('9', 'H', 9), Card('K', 'H', 13), Card('2', 'D', 2), Card('Q', 'H', 12), Card('7', 'S', 7), Card('J', 'H', 11), Card('_', 'H', 10)];
        let low_straight: Vec<Card> = vec![Card('A', 'H', 14), Card('3', 'H', 3), Card('2', 'H', 2), Card('Q', 'D', 12), Card('5', 'H', 5), Card('J', 'S', 11), Card('4', 'H', 4)];
        assert_eq!(calc_rank(&high_straight), StraightFlush);
        assert_eq!(calc_rank(&low_straight), StraightFlush);
    }

    #[test]
    fn test_four_of_kind() {
        let hand: Vec<Card> = vec![Card('9', 'H', 9), Card('2', 'D', 2), Card('9', 'S', 9), Card('9', 'D', 9), Card('2', 'H', 2), Card('9', 'C', 9), Card('2', 'C', 2)];
        assert_eq!(calc_rank(&hand), FourOfKind);
    }

    #[test]
    fn test_full_house() {
        let hand1: Vec<Card> = vec![Card('9', 'H', 9), Card('2', 'D', 2), Card('9', 'S', 9), Card('9', 'D', 9), Card('2', 'H', 2), Card('3', 'C', 3), Card('2', 'C', 2)];
        let hand2: Vec<Card> = vec![Card('3', 'H', 3), Card('2', 'D', 2), Card('9', 'S', 9), Card('9', 'D', 9), Card('2', 'H', 2), Card('3', 'C', 3), Card('2', 'C', 2)];
        assert_eq!(calc_rank(&hand1), FullHouse);
        assert_eq!(calc_rank(&hand2), FullHouse);
    }

    #[test]
    fn test_flush() {
        let hand: Vec<Card> = vec![Card('9', 'H', 9), Card('K', 'H', 13), Card('2', 'D', 2), Card('2', 'H', 2), Card('7', 'S', 7), Card('J', 'H', 11), Card('_', 'H', 10)];
        assert_eq!(calc_rank(&hand), Flush)
    }

    #[test]
    fn test_straight() {
        let straight_high: Vec<Card> = vec![Card('9', 'D', 9), Card('K', 'C', 13), Card('2', 'D', 2), Card('Q', 'H', 12), Card('7', 'S', 7), Card('J', 'H', 11), Card('_', 'H', 10)];
        let straight_low: Vec<Card> = vec![Card('4', 'D', 4), Card('A', 'C', 14), Card('2', 'D', 2), Card('Q', 'H', 12), Card('5', 'S', 5), Card('J', 'H', 11), Card('3', 'H', 3)];
        assert_eq!(calc_rank(&straight_high), Straight);
        assert_eq!(calc_rank(&straight_low), Straight);
    }

    #[test]
    fn test_three_of_kind() {
        let hand: Vec<Card> = vec![Card('9', 'H', 9), Card('7', 'D', 7), Card('3', 'S', 3), Card('9', 'D', 9), Card('2', 'H', 2), Card('9', 'C', 9), Card('5', 'C', 5)];
        assert_eq!(calc_rank(&hand), ThreeOfKind);
    }

    #[test]
    fn test_two_pair() {
        let hand: Vec<Card> = vec![Card('9', 'H', 9), Card('2', 'D', 2), Card('9', 'S', 9), Card('8', 'D', 8), Card('2', 'H', 2), Card('6', 'C', 6), Card('4', 'C', 4)];
        assert_eq!(calc_rank(&hand), TwoPair);
    }

    #[test]
    fn test_one_pair() {
        let hand: Vec<Card> = vec![Card('A', 'H', 14), Card('2', 'D', 2), Card('9', 'S', 9), Card('8', 'D', 8), Card('2', 'H', 2), Card('6', 'C', 6), Card('4', 'C', 4)];
        assert_eq!(calc_rank(&hand), OnePair);
    }

    #[test]
    fn test_high_card() {
        let hand: Vec<Card> = vec![Card('A', 'H', 14), Card('J', 'D', 11), Card('9', 'S', 9), Card('8', 'D', 8), Card('2', 'H', 2), Card('6', 'C', 6), Card('4', 'C', 4)];
        assert_eq!(calc_rank(&hand), HighCard);
    }
}
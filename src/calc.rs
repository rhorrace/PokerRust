use crate::card::Card;
use crate::calc::Rank::*;
use std::collections::HashMap;

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

pub fn highest_rank(hand: &[Card]) -> Rank {
    let mut suit_count: HashMap<char, u8> = HashMap::new();
    let mut values: Vec<u8> = Vec::new();
    if is_flush(&hand, &mut suit_count) {
        let max_suit = *suit_count.iter()
            .max_by_key(|entry| entry.1)
            .unwrap().0;
        let flush: Vec<Card> = hand.iter()
            .filter(|&c| c.1 == max_suit)
            .map(|c| c.clone())
            .collect();
        if is_straight(&flush, &mut values) {
            if values[0] == 14 && values[4] == 10 {
                RoyalFlush
            } else {
                StraightFlush
            }
        } else {
            Flush
        }
    } else if is_straight(&hand, &mut values) {
        Straight
    } else {
        HighCard
    }
}

fn is_flush(hand: &[Card], suit_count: &mut HashMap<char, u8>) -> bool {
    for card in hand {
        *suit_count.entry(card.1).or_insert(0) += 1;
    }
    *suit_count.values()
        .max()
        .unwrap() >= 5
}

fn is_straight(hand: &[Card], values: &mut Vec<u8>) -> bool {
    let mut card_values: Vec<u8> = hand.iter()
        .map(|c| c.2)
        .collect();
    card_values.sort();
    if card_values.last() == Some(&14) {
        values.push(1);
    }
    values.append(&mut card_values);
    values.dedup();
    values.reverse();
    let mut count = 1;
    let mut prev = values[0];
    for &i in values.iter().skip(1) {
        if i == prev - 1 {
            count += 1;
            if count == 5 {
                return true;
            }
        } else {
            count = 1;
        }
        prev = i;
    }
    false
}

fn is_other(hand: &[Card]) -> Rank {
    let mut value_count: HashMap<u8,u8> = HashMap::new();
    let mut counts: HashMap<u8,u8> = (1..=4).map(|i| (i, 0)).collect::<HashMap<_,_>>();
    for card in hand {
        *value_count.entry(card.2).or_insert(0) += 1;
    }
    for &i in value_count.values() {
        *counts.entry(i).or_insert(0) += 1;
    }
    if counts[&4] >= 1 {
        FourOfKind
    } else if counts[&3] >= 1 {
        if counts[&3] >= 2 || counts[&2] >= 1 {
            FullHouse
        } else {
            ThreeOfKind
        }
    } else if counts[&2] >= 2 {
        TwoPair
    } else if counts[&2] == 1 {
        OnePair
    } else {
        HighCard
    }
}
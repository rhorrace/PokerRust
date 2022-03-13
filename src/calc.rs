use crate::calc::Rank::*;
use crate::card::{Card, Name, Name::*, Suit};
use std::collections::{BTreeSet, HashMap, HashSet};
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
        .for_each(|card| *value_count.entry(card.0).or_insert(0) += 1);

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
        let suit: Suit = mode_suit(&cards).unwrap();
        cards = cards.into_iter()
            .filter(|c| c.1 == suit)
            .collect();
    }

    // Get best hand depending on rank
    match rank {
        HighCard | OnePair | ThreeOfKind | Flush | RoyalFlush =>
            cards.into_iter()
                .take(5)
                .collect(),
        TwoPair | FourOfKind => {
            let mut best_hand: Vec<Card> = cards[0..4].to_vec();
            let mut kicker: Vec<Card> = cards.into_iter()
                .skip(4)
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
            let mut pairs: Vec<Card> = cards.into_iter()
                .skip(3)
                .filter(|&c| value_count[&c.0] >= 2)
                .collect();

            pairs.sort_by(|a, b| b.cmp(a)
                .then_with(|| a.1.cmp(&b.1)));
            best_hand.extend(&pairs[..2]);
            best_hand
        }
    }
}

// Calculates the rank in a given array(hand) of cards
pub fn calc_rank(hand: &[Card]) -> Rank {
    if let Some(rank) = check_flush(&hand) {
        rank
    } else if is_straight(&hand) {
        Straight
    } else {
        other_rank(&hand)
    }
}

// Checks if ahd is a flush, if so, what type of flush
fn check_flush(hand: &[Card]) -> Option<Rank> {
    let some_suit: Option<Suit> = mode_suit(&hand);

    if some_suit == None || hand.len() < 5 {
        return None;
    }

    let suit: Suit = some_suit.unwrap();
    let flush: Vec<Card> = hand.iter()
        .filter(|&c| c.1 == suit)
        .map(|&c| c)
        .collect();

    if flush.len() < 5 {
        return None;
    }

    if !is_straight(&flush) {
        return Some(Flush);
    }

    let names: HashSet<Name> = hand.iter()
        .map(|card| card.0)
        .collect();
    let royal_flush: HashSet<Name> = HashSet::from([Ten, Jack, Queen, King, AceHigh]);

    match royal_flush.is_subset(&names) {
        true => Some(RoyalFlush),
        false => Some(StraightFlush)
    }
}

// Checks if hand is a straight
fn is_straight(hand: &[Card]) -> bool {
    if hand.len() < 5 {
        return false;
    }

    let mut names: BTreeSet<Name> = hand.iter()
        .map(|card| card.0)
        .collect();

    if names.contains(&AceHigh) {
        names.insert(AceLow);
    }

    names.into_iter()
        .collect::<Vec<Name>>()
        .windows(5)
        .any(|straight| straight[4] as u8 - straight[0] as u8 == 4)
}

// Finds the most frequent suit in the array of cards
fn mode_suit(cards: &[Card]) -> Option<Suit> {
    let mut suit_count: HashMap<Suit, u8> = HashMap::new();

    cards.iter()
        .map(|card| card.1)
        .max_by_key(|&suit| {
            let count: &mut u8 = suit_count.entry(suit).or_insert(0);
            *count += 1;
            *count
        })
}

// Calculates non-flush/non-straight rank
fn other_rank(hand: &[Card]) -> Rank {
    let mut value_count: HashMap<Name, u8> = HashMap::new();
    let mut counts: HashMap<u8, u8> = (1..=4).map(|i| (i, 0)).collect();

    hand.iter()
        .for_each(|card| *value_count.entry(card.0).or_insert(0) += 1);
    value_count.values()
        .for_each(|&i| *counts.entry(i).or_insert(0) += 1);

    if counts[&4] > 0 {
        return FourOfKind;
    }

    match (counts[&3], counts[&2]) {
        (0, 0) => HighCard,
        (0, 1) => OnePair,
        (0, _) => TwoPair,
        (1, 0) => ThreeOfKind,
        (_, _) => FullHouse
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
                                   Card(Ten, Hearts),
                                   Card(Seven, Spades),
                                   Card(Jack, Hearts),
                                   Card(Queen, Hearts)];
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
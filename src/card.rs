use std::cmp::{Ordering};
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Name {
    AceLow = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    AceHigh = 14,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Suit {
    Hearts = 1,
    Diamonds = 2,
    Spades = 3,
    Clubs = 4,
}

#[derive(Clone, Copy)]
pub struct Card(pub Name, pub Suit);

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        custom_fmt(*self, f)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        custom_fmt(*self, f)
    }
}

impl Eq for Card {}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }

    fn lt(&self, other: &Self) -> bool {
        self.0 < other.0
    }

    fn le(&self, other: &Self) -> bool {
        self.0 <= other.0
    }

    fn gt(&self, other: &Self) -> bool {
        self.0 > other.0
    }

    fn ge(&self, other: &Self) -> bool {
        self.0 >= other.0
    }
}

fn custom_fmt(card: Card, f: &mut Formatter<'_>) -> std::fmt::Result {
    let name: &str = match card.0 {
        Name::AceLow | Name::AceHigh => "A",
        Name::Two => "2",
        Name::Three => "3",
        Name::Four => "4",
        Name::Five => "5",
        Name::Six => "6",
        Name::Seven => "7",
        Name::Eight => "8",
        Name::Nine => "9",
        Name::Ten => "10",
        Name::Jack => "J",
        Name::Queen => "Q",
        Name::King => "K",
    };
    let suit: char = match card.1 {
        Suit::Hearts => 'H',
        Suit::Diamonds => 'D',
        Suit::Spades => 'S',
        Suit::Clubs => 'C'
    };
    write!(f, "{}:{}", name, suit)
}

#[cfg(test)]
mod card_tests {
    use crate::card::{Card, Name::*, Suit::*};
    use std::cmp::Ordering;

    #[test]
    fn test_new() {
        let card: Card = Card(AceHigh, Hearts);
        assert_eq!(card.0, AceHigh);
        assert_eq!(card.1, Hearts);
        assert_eq!(card.0 as u8, 14);
    }

    #[test]
    fn test_cmp() {
        let card1: Card = Card(Ten, Hearts);
        let card2: Card = Card(Ten, Diamonds);
        let card3: Card = Card(Five, Diamonds);
        let card4: Card = Card(AceHigh, Diamonds);
        assert_eq!(card1.cmp(&card2), Ordering::Equal);
        assert_eq!(card1.cmp(&card3), Ordering::Greater);
        assert_eq!(card1.cmp(&card4), Ordering::Less);
    }

    #[test]
    fn test_eq() {
        let card1: Card = Card(Ten, Hearts);
        let card2: Card = Card(Ten, Diamonds);
        let card3: Card = Card(Five, Diamonds);
        assert!(card1 == card2);
        assert_eq!(card1 == card3, false);
    }

    #[test]
    fn test_ne() {
        let card1: Card = Card(Ten, Hearts);
        let card2: Card = Card(Ten, Diamonds);
        let card3: Card = Card(Five, Diamonds);
        assert_eq!(card1 != card2, false);
        assert!(card1 != card3);
    }

    #[test]
    fn test_partial_cmp() {
        let card1: Card = Card(Ten, Hearts);
        let card2: Card = Card(Ten, Diamonds);
        let card3: Card = Card(Five, Diamonds);
        let card4: Card = Card(AceHigh, Diamonds);
        assert_eq!(card1.partial_cmp(&card2), Some(Ordering::Equal));
        assert_eq!(card1.partial_cmp(&card3), Some(Ordering::Greater));
        assert_eq!(card1.partial_cmp(&card4), Some(Ordering::Less));
    }

    #[test]
    fn test_lt() {
        let card1: Card = Card(Ten, Hearts);
        let card2: Card = Card(Jack, Diamonds);
        let card3: Card = Card(Five, Diamonds);
        assert!(card1 < card2);
        assert_eq!(card1 < card3, false);
    }

    #[test]
    fn test_le() {
        let card1: Card = Card(Ten, Hearts);
        let card2: Card = Card(Ten, Diamonds);
        let card3: Card = Card(Jack, Diamonds);
        let card4: Card = Card(Five, Diamonds);
        assert!(card1 <= card2);
        assert!(card1 <= card3);
        assert_eq!(card1 <= card4, false);
    }

    #[test]
    fn test_gt() {
        let card1: Card = Card(Ten, Hearts);
        let card2: Card = Card(Five, Diamonds);
        let card3: Card = Card(Jack, Diamonds);
        assert!(card1 > card2);
        assert_eq!(card1 > card3, false);
    }

    #[test]
    fn test_ge() {
        let card1: Card = Card(Ten, Hearts);
        let card2: Card = Card(Ten, Diamonds);
        let card3: Card = Card(Five, Diamonds);
        let card4: Card = Card(Jack, Diamonds);
        assert!(card1 >= card2);
        assert!(card1 >= card3);
        assert_eq!(card1 >= card4, false);
    }
}
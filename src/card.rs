use std::fmt::{Display, Formatter, Debug};
use std::cmp::{Ordering};

#[derive(Clone, Copy)]
pub struct Card(pub char, pub char, pub u8);

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
        self.2.cmp(&other.2)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.2 == other.2
    }

    fn ne(&self, other: &Self) -> bool {
        self.2 != other.2
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.2.partial_cmp(&other.2)
    }

    fn lt(&self, other: &Self) -> bool {
        self.2 < other.2
    }

    fn le(&self, other: &Self) -> bool {
        self.2 <= other.2
    }

    fn gt(&self, other: &Self) -> bool {
        self.2 > other.2
    }

    fn ge(&self, other: &Self) -> bool {
        self.2 >= other.2
    }
}

fn custom_fmt(card: Card, f: &mut Formatter<'_>) -> std::fmt::Result {
    match card.2 {
        10 => write!(f, "{}:{}", card.2, card.1),
        _ => write!(f, "{}:{}", card.0, card.1)
    }
}

#[cfg(test)]
mod card_tests {
    use crate::card::Card;
    use std::cmp::Ordering;

    #[test]
    fn test_new() {
        let card: Card = Card('A', 'H', 14);
        assert_eq!(card.0, 'A');
        assert_eq!(card.1, 'H');
        assert_eq!(card.2, 14);
    }

    #[test]
    fn test_cmp() {
        let card1: Card = Card('_', 'H', 10);
        let card2: Card = Card('_', 'D', 10);
        let card3: Card = Card('5', 'D', 5);
        let card4: Card = Card('A', 'D', 14);
        assert_eq!(card1.cmp(&card2), Ordering::Equal);
        assert_eq!(card1.cmp(&card3), Ordering::Greater);
        assert_eq!(card1.cmp(&card4), Ordering::Less);
    }

    #[test]
    fn test_eq() {
        let card1: Card = Card('_', 'H', 10);
        let card2: Card = Card('_', 'D', 10);
        let card3: Card = Card('5', 'D', 5);
        assert!(card1 == card2);
        assert_eq!(card1 == card3, false);
    }

    #[test]
    fn test_ne() {
        let card1: Card = Card('_', 'H', 10);
        let card2: Card = Card('_', 'D', 10);
        let card3: Card = Card('5', 'D', 5);
        assert_eq!(card1 != card2, false);
        assert!(card1 != card3);
    }

    #[test]
    fn test_partial_cmp() {
        let card1: Card = Card('_', 'H', 10);
        let card2: Card = Card('_', 'D', 10);
        let card3: Card = Card('5', 'D', 5);
        let card4: Card = Card('A', 'D', 14);
        assert_eq!(card1.partial_cmp(&card2), Some(Ordering::Equal));
        assert_eq!(card1.partial_cmp(&card3), Some(Ordering::Greater));
        assert_eq!(card1.partial_cmp(&card4), Some(Ordering::Less));
    }

    #[test]
    fn test_lt() {
        let card1: Card = Card('_', 'H', 10);
        let card2: Card = Card('J', 'D', 11);
        let card3: Card = Card('5', 'D', 5);
        assert!(card1 < card2);
        assert_eq!(card1 < card3, false);
    }

    #[test]
    fn test_le() {
        let card1: Card = Card('_', 'H', 10);
        let card2: Card = Card('_', 'D', 10);
        let card3: Card = Card('J', 'D', 11);
        let card4: Card = Card('5', 'D', 5);
        assert!(card1 <= card2);
        assert!(card1 <= card3);
        assert_eq!(card1 <= card4, false);
    }

    #[test]
    fn test_gt() {
        let card1: Card = Card('_', 'H', 10);
        let card2: Card = Card('5', 'D', 5);
        let card3: Card = Card('J', 'D', 11);
        assert!(card1 > card2);
        assert_eq!(card1 > card3, false);
    }

    #[test]
    fn test_ge() {
        let card1: Card = Card('_', 'H', 10);
        let card2: Card = Card('_', 'D', 10);
        let card3: Card = Card('5', 'D', 5);
        let card4: Card = Card('J', 'D', 11);
        assert!(card1 >= card2);
        assert!(card1 >= card3);
        assert_eq!(card1 >= card4, false);
    }
}
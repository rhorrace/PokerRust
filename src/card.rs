use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Card {
    name: &'static str,
    suit: char,
    value: u8,
}

impl Card {
    pub fn new(name: &'static str, suit: char, value: u8) -> Card {
        Card {
            name,
            suit,
            value,
        }
    }

    pub fn is(self, name: &str) -> bool {
        self.name.eq(name)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn ne(&self, other: &Self) -> bool {
        self.value != other.value
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }

    fn lt(&self, other: &Self) -> bool {
        self.value < other.value
    }

    fn le(&self, other: &Self) -> bool {
        self.value <= other.value
    }

    fn gt(&self, other: &Self) -> bool {
        self.value > other.value
    }

    fn ge(&self, other: &Self) -> bool {
        self.value >= other.value
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.name, self.suit)
    }
}
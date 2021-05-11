use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Card(pub &'static str, pub char, pub u8);

impl Card {
    pub fn new(name: &'static str, suit: char, value: u8) -> Card {
        Card(name, suit, value)
    }

    pub fn is(self, name: &str) -> bool {
        self.0.eq(name)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}
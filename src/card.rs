pub const SUITS: [Suit; 4] = [Suit::Hearts, Suit::Spades, Suit::Diamonds, Suit::Clubs];
/// A lowest card is 2.
pub const INITIAL_VALUE: u8 = 2;
/// A highest card is Ace with value of 14.
pub const HIGHEST_VALUE: u8 = 14;

#[derive(Clone)]
pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

pub struct Card {
    suit: Suit,
    value: u8,
    letter: char,
    emoji: &'static str,
}

impl Card {
    pub fn new(suit: Suit, value: u8) -> Self {
        let (letter, emoji) = match suit {
            Suit::Hearts => ('H', "♥️"),
            Suit::Spades => ('S', "♠️"),
            Suit::Diamonds => ('D', "♦️"),
            Suit::Clubs => ('C', "♣️"),
        };

        Self {
            suit,
            value,
            letter,
            emoji,
        }
    }
}

impl Card {
    pub fn get_suit(&self) -> &Suit {
        &self.suit
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn get_letter(&self) -> char {
        self.letter
    }

    pub fn get_emoji(&self) -> &str {
        &self.emoji
    }

    pub fn is_ace(&self) -> bool {
        self.value == 14
    }

    pub fn is_joker(&self) -> bool {
        self.value == 0
    }
}

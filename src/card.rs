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
    emoji: String,
    // "is_ace" field's needed so that calling code can check
    // if it should combinations with both "1" and "14" values.
    is_ace: bool,
}

impl Card {
    pub fn new(suit: Suit, value: u8) -> Self {
        let is_ace: bool = value == 14;

        let (letter, emoji) = match suit {
            Suit::Hearts => ('H', "♥️".to_string()),
            Suit::Spades => ('S', "♠️".to_string()),
            Suit::Diamonds => ('D', "♦️".to_string()),
            Suit::Clubs => ('C', "♣️".to_string()),
        };

        Self {
            suit,
            value,
            is_ace,
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
        self.is_ace
    }
}

// suit emojis: ♥️♠️♦️♣️
pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

pub struct Card {
    suit: Suit,
    value: u8,
    // "is_ace" field's needed so that calling code can check
    // if it should combinations with both "1" and "14" values.
    is_ace: bool,
}

impl Card {
    pub fn new(suit: Suit, value: u8) -> Self {
        let is_ace: bool;
        if value == 14 {
            is_ace = true;
        } else {
            is_ace = false;
        }

        Self {
            suit,
            value,
            is_ace,
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

    pub fn is_ace(&self) -> bool {
        self.is_ace
    }
}

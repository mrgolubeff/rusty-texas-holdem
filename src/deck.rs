use std::vec::Vec;
use crate::card::{Card, Suit};

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn construct_deck() -> Self {
        let mut cards = Vec::new();

        'suits: for suit_number in 0..4 {
            'values: for value in 2..=14 {
                let suit: Suit = match suit_number {
                    0 => Suit::Hearts,
                    1 => Suit::Spades,
                    2 => Suit::Diamonds,
                    3 => Suit::Clubs,
                    val => panic!("suit_number should be 0, 1, 2 or 3, but value is {val}.")
                };
                cards.push(
                    Card::new(suit, value)
                );
            }
        }

        Self { cards }
    }
}

use crate::card::{Card, HIGHEST_VALUE, INITIAL_VALUE, SUITS};
use rand::rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::vec::Vec;

pub struct Deck {
    cards: Vec<Card>,
    potato: ThreadRng, // Random number generator. If you know, you know.
}

impl Deck {
    pub fn construct_deck() -> Self {
        let mut cards = Vec::new();

        for suit in SUITS {
            for value in INITIAL_VALUE..=HIGHEST_VALUE {
                cards.push(Card::new(suit.clone(), value));
            }
        }

        Self {
            cards,
            potato: rng(),
        }
    }
}

impl Deck {
    pub fn shuffle_deck(&mut self) {
        self.cards.shuffle(&mut self.potato);
    }
}

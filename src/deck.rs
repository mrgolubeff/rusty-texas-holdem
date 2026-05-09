use crate::card::{Card, HIGHEST_VALUE, INITIAL_VALUE, SUITS};
use rand::rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

pub struct Deck {
    cards: Vec<Card>,
    rng: ThreadRng,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();

        for suit in SUITS {
            for value in INITIAL_VALUE..=HIGHEST_VALUE {
                cards.push(Card::new(suit.clone(), value));
            }
        }

        Self { cards, rng: rng() }
    }
}

impl Deck {
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut self.rng);
    }
}

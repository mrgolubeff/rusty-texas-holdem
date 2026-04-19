use crate::card::Card;

pub enum Hand {
    RoyalFlush(u16),
    StraightFlush(u16),
    FourOfAKind(u16),
    FullHouse(u16),
    Flush(u16),
    Straight(u16),
    ThreeOfAKind(u16),
    TwoPair(u16),
    Pair(u16),
    HighCard(u16),
}

impl Hand {
    // retrieve value from any of the variants.
    pub fn get_score(&self) -> u16 {
        match self {
            Self::RoyalFlush(value)
            | Self::StraightFlush(value)
            | Self::FourOfAKind(value)
            | Self::FullHouse(value)
            | Self::Flush(value)
            | Self::Straight(value)
            | Self::ThreeOfAKind(value)
            | Self::TwoPair(value)
            | Self::Pair(value)
            | Self::HighCard(value) => *value
        }
    }
}

// pub struct PlayerHands {
//     cards: Vec<&Card>,
//     hands: Vec<Hand>,
// }

// impl PlayerHands {
//     pub fn make_hands(Vec<&Card>) -> Self {}
// }

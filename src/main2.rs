use rand::prelude::*;

const SUITS: [char; 4] = ['H', 'S', 'D', 'C'];
const VALUE: [i8; 13] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];

fn main() {
    let deck: Vec<Card> = construct_deck();
    for i in 1..=10 {
        let card = get_random_card(&deck);
        println!("Card {}: {}", i, card);
    }
}

fn construct_deck() -> Vec<Card> {
    let mut deck = Vec::new();

    'suits: for suit in SUITS {
        'values: for value in VALUE {
            deck.push(Card { suit, value })
        }
    }

    deck
}

fn get_random_card(deck: &Vec<Card>) -> String {
    let mut rng = rand::rng();
    let random_index = rng.random_range(0..52);
    let random_card: &Card = &deck[random_index];
    format!(
        "{}{}",
        random_card.get_suit_emoji(),
        random_card.get_facecard()
    )
}

enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

struct Card {
    suit: char,
    value: i8,
}

impl Card {
    fn get_facecard(&self) -> String {
        match self.value {
            11 => format!("{}", "J"),
            12 => format!("{}", "Q"),
            13 => format!("{}", "K"),
            14 => format!("{}", "A"),
            _ => format!("{}", self.value),
        }
    }

    fn get_suit_emoji(&self) -> String {
        match self.suit {
            suit if suit == SUITS[0] => format!("{}", "♥️"),
            suit if suit == SUITS[1] => format!("{}", "♠️"),
            suit if suit == SUITS[2] => format!("{}", "♦️"),
            suit if suit == SUITS[3] => format!("{}", "♣️"),
            _ => format!("?"),
        }
    }
}

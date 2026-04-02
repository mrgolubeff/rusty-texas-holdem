use rand::prelude::*;

fn main() {
    let deck: Vec<Card> = construct_deck();
    for i in 1..=5 {
        let card = get_random_card(&deck);
        println!("Card {}: {}", i, card);
    }
}

fn construct_deck() -> Vec<Card> {
    const SUITS: [char; 4] = ['H', 'S', 'D', 'C'];
    const VALUE: [i8; 13] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];

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
    format!("{}{}", random_card.suit, random_card.value)
}

struct Card {
    suit: char,
    value: i8,
}

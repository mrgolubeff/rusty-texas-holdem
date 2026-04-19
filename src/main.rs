use rusty_holdem::deck::Deck;

fn main() {
    let mut deck = Deck::construct_deck();
    println!("{:?}\n\n\n", deck);
    deck.shuffle_deck();
    println!("{:?}", deck);
}

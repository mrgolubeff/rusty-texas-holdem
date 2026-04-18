pub mod card;
pub mod deck;
pub mod hands;
pub mod pile;

fn main() {
    let mut deck = crate::deck::Deck::construct_deck();
    println!("{:?}\n\n\n", deck);
    deck.shuffle_deck();
    println!("{:?}", deck);
}

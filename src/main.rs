use hanabi_tui::{deck::Deck, hand::Hand};

fn main() {
    let mut deck = Deck::default();
    let hand = Hand::draw(&mut deck);

    println!("{:#?}", hand);
    println!("{:#?}", deck.len());
    println!("{:#?}", deck.is_empty());
}

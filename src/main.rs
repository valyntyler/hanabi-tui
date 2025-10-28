use hanabi_tui::{card, deck::Deck, hand::Hand};

fn main() {
    let _card = card!("red", 2);
    let _deck = Deck::default();
    let _hand = Hand::new([_card; 5]);

    println!("{:#?}", _deck);
}

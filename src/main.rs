use hanabi_tui::{card, deck::Deck, hand::Hand};

fn main() {
    let mut _deck = Deck::default();

    let _card = card!("red", 2);
    let _hand = Hand::new(vec![
        _deck.draw().unwrap(),
        _deck.draw().unwrap(),
        _deck.draw().unwrap(),
        _deck.draw().unwrap(),
        _deck.draw().unwrap(),
    ]);

    println!("{:#?}", _hand);
}

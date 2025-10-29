use hanabi_tui::{deck::Deck, hand::Hand, pile::discard::DiscardPile};

fn main() {
    let mut deck = Deck::default();
    let mut hand = Hand::empty();

    hand.draw(&mut deck);
    hand.draw(&mut deck);

    let mut discard = DiscardPile::empty();
    hand.discard(&mut discard, 0);
    hand.discard(&mut discard, 0);
    hand.discard(&mut discard, 0);

    println!("{:#?}", hand);
    println!("{:#?}", deck.len());
    println!("{:#?}", deck.is_empty());
}

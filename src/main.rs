use hanabi_tui::{
    area::{discard::DiscardArea, scoring::ScoringArea},
    deck::Deck,
    hand::Hand,
};

fn main() {
    let mut deck = Deck::default();
    let mut hand = Hand::empty();

    hand.draw(&mut deck);
    hand.draw(&mut deck);

    let mut discard = DiscardArea::empty();
    let mut scoring = ScoringArea::default();

    hand.play(&mut scoring, &mut discard, 0);
    hand.play(&mut scoring, &mut discard, 0);

    println!("{:#?}", hand);
    println!("{:#?}", scoring);
    println!("{:#?}", discard);
}

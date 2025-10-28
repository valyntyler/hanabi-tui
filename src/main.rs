use hanabi_tui::{
    card::{Card, color::CardColor, value::CardValue},
    hand::Hand,
};

fn main() {
    let _card = Card::new(CardColor::Red, CardValue::_5);
    let _hand = Hand::new([_card; 5]);

    println!("{:?}", _hand);
}

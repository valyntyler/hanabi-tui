use hanabi_tui::{
    card::{Card, color::CardColor, value::CardValue},
    card_color,
    hand::Hand,
};

fn main() {
    let _card = Card::new(card_color!("blue"), CardValue::_5);
    let _hand = Hand::new([_card; 5]);

    println!("{:?}", _hand);
}

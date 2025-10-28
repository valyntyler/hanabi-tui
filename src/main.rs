use hanabi_tui::{
    card::{Card, color::CardColor, value::CardValue},
    card_color, card_value,
    hand::Hand,
};

fn main() {
    let _card = Card::new(card_color!("blue"), card_value!(3));
    let _hand = Hand::new([_card; 5]);

    println!("{:?}", _hand);
}

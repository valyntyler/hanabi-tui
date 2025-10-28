use hanabi_tui::card::{Card, color::CardColor, value::CardValue};

fn main() {
    let card = Card::new(CardColor::Red, CardValue::_5);

    println!("{:?}", card);
}

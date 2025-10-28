use hanabi_tui::{card, hand::Hand};

fn main() {
    let _card = card!("red", 2);
    let _hand = Hand::new([_card; 5]);

    println!("{:?}", _hand);
}

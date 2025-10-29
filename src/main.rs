use hanabi_tui::{Render, card, hand::Hand};

fn main() {
    let hand = Hand::new(vec![
        card!("red", 1),
        card!("blue", 2),
        card!("green", 3),
        card!("white", 4),
        card!("yellow", 5),
    ]);

    println!("{}", hand.render());
    println!("{}", card!("yellow", 5).render());
}

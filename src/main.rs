use hanabi_tui::{Render, card};

fn main() {
    let card = card!("red", 1);
    println!("{}", card.render());

    let card = card!("blue", 2);
    println!("{}", card.render());

    let card = card!("green", 3);
    println!("{}", card.render());

    let card = card!("white", 4);
    println!("{}", card.render());

    let card = card!("yellow", 5);
    println!("{}", card.render());
}

use crate::{card::Card, deck::Deck, hand::Hand};

#[allow(dead_code)]
pub struct Game {
    hands: Vec<Hand>,
    cards: Deck,

    stacked: [i8; 5],
    discard: Vec<Card>,

    info_tokens: i8,
    fuse_tokens: i8,
}

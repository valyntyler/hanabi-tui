use crate::{
    area::{discard::DiscardArea, scoring::ScoringArea},
    deck::Deck,
    hand::Hand,
};

#[allow(dead_code)]
pub struct Game {
    hands: Vec<Hand>,
    cards: Deck,

    scoring: ScoringArea,
    discard: DiscardArea,

    info_tokens: i8,
    fuse_tokens: i8,
}

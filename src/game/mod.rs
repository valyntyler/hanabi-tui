use crate::{area::scoring::ScoringArea, deck::Deck, hand::Hand, pile::discard::DiscardPile};

#[allow(dead_code)]
pub struct Game {
    hands: Vec<Hand>,
    cards: Deck,

    scoring: ScoringArea,
    discard: DiscardPile,

    info_tokens: i8,
    fuse_tokens: i8,
}

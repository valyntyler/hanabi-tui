use crate::card::{color::CardColor, value::CardValue};

#[allow(dead_code)]
pub struct ScoringPile {
    score: CardValue,
    color: CardColor,
}

impl ScoringPile {
    pub fn new(score: CardValue, color: CardColor) -> Self {
        Self { score, color }
    }
}

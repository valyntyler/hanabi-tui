use crate::card::{color::CardColor, value::CardValue};

#[allow(dead_code)]
pub struct ScoringPile {
    score: Option<CardValue>,
    color: CardColor,
}

impl ScoringPile {
    pub fn new(score: Option<CardValue>, color: CardColor) -> Self {
        Self { score, color }
    }

    pub fn top(&self) -> Option<CardValue> {
        self.score
    }
}

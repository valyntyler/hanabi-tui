use std::collections::HashMap;

use crate::card::{Card, color::CardColor, value::CardValue};

#[derive(Debug, Clone, Copy, Default)]
pub struct PileScore(pub Option<CardValue>);

impl PileScore {
    pub fn next(&self) -> Option<CardValue> {
        match self.0 {
            None => Some(CardValue::_1),
            Some(CardValue::_1) => Some(CardValue::_2),
            Some(CardValue::_2) => Some(CardValue::_3),
            Some(CardValue::_3) => Some(CardValue::_4),
            Some(CardValue::_4) => Some(CardValue::_5),
            Some(CardValue::_5) => None,
        }
    }
}

#[derive(Debug)]
pub struct ScoringArea(HashMap<CardColor, PileScore>);

impl Default for ScoringArea {
    fn default() -> Self {
        Self(HashMap::from(
            CardColor::all().map(|color| (color, PileScore::default())),
        ))
    }
}

impl ScoringArea {
    pub fn get(&self, color: CardColor) -> PileScore {
        *self.0.get(&color).unwrap()
    }

    pub fn set(&mut self, color: CardColor, value: PileScore) {
        *self.0.get_mut(&color).unwrap() = value;
    }

    pub fn play(&mut self, card: Card) -> Result<(), Card> {
        match self.get(card.color).next() {
            None => Err(card),
            Some(score) => match card.value == score {
                false => Err(card),
                true => {
                    self.set(card.color, PileScore(Some(card.value)));
                    Ok(())
                }
            },
        }
    }
}

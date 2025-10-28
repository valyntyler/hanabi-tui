use crate::card::color::CardColor;
use crate::card::value::CardValue;

pub mod color;
pub mod value;

#[derive(Debug, Clone, Copy)]
pub struct Card {
    color: CardColor,
    value: CardValue,
}

impl Card {
    pub fn new(color: CardColor, value: CardValue) -> Self {
        Self { color, value }
    }
}

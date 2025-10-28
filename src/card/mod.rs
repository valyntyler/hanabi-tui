use crate::card::color::CardColor;
use crate::card::value::CardValue;

pub mod color;
pub mod value;

#[macro_export]
macro_rules! card {
    ($color:tt, $value:tt) => {
        $crate::card::Card::new($crate::card_color!($color), $crate::card_value!($value))
    };
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Card {
    color: CardColor,
    value: CardValue,
}

impl Card {
    pub fn new(color: CardColor, value: CardValue) -> Self {
        Self { color, value }
    }
}

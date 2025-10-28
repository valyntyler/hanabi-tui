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
pub struct Card {
    _color: CardColor,
    _value: CardValue,
}

impl Card {
    pub fn new(_color: CardColor, _value: CardValue) -> Self {
        Self { _color, _value }
    }
}

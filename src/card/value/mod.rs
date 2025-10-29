use std::fmt::Display;

#[macro_export]
macro_rules! card_value {
    (1) => {
        $crate::card::value::CardValue::_1
    };
    (2) => {
        $crate::card::value::CardValue::_2
    };
    (3) => {
        $crate::card::value::CardValue::_3
    };
    (4) => {
        $crate::card::value::CardValue::_4
    };
    (5) => {
        $crate::card::value::CardValue::_5
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardValue {
    _1,
    _2,
    _3,
    _4,
    _5,
}

impl From<CardValue> for u8 {
    fn from(value: CardValue) -> Self {
        match value {
            CardValue::_1 => 1,
            CardValue::_2 => 2,
            CardValue::_3 => 3,
            CardValue::_4 => 4,
            CardValue::_5 => 5,
        }
    }
}

impl Display for CardValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u8::from(*self))
    }
}

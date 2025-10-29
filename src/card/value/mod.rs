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

impl CardValue {
    pub fn next(&self) -> Option<Self> {
        match self {
            CardValue::_1 => Some(CardValue::_2),
            CardValue::_2 => Some(CardValue::_3),
            CardValue::_3 => Some(CardValue::_4),
            CardValue::_4 => Some(CardValue::_5),
            CardValue::_5 => None,
        }
    }
}

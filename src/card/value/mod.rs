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

#[derive(Debug, Clone, Copy)]
pub enum CardValue {
    _1,
    _2,
    _3,
    _4,
    _5,
}

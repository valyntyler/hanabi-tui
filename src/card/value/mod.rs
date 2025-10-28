#[macro_export]
macro_rules! card_value {
    (1) => {
        CardValue::_1
    };
    (2) => {
        CardValue::_2
    };
    (3) => {
        CardValue::_3
    };
    (4) => {
        CardValue::_4
    };
    (5) => {
        CardValue::_5
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

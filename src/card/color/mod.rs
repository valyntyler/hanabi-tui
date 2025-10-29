#[macro_export]
macro_rules! card_color {
    ("blue") => {
        $crate::card::color::CardColor::Blue
    };
    ("green") => {
        $crate::card::color::CardColor::Green
    };
    ("red") => {
        $crate::card::color::CardColor::Red
    };
    ("white") => {
        $crate::card::color::CardColor::White
    };
    ("yellow") => {
        $crate::card::color::CardColor::Yellow
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardColor {
    Blue,
    Green,
    Red,
    White,
    Yellow,
}

impl CardColor {
    pub fn all() -> [Self; 5] {
        [
            Self::Blue,
            Self::Green,
            Self::Red,
            Self::Yellow,
            Self::White,
        ]
    }
}

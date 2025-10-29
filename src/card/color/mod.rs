use std::fmt::Display;

use colored::Colorize;

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

impl From<CardColor> for colored::Color {
    fn from(value: CardColor) -> Self {
        match value {
            CardColor::Blue => colored::Color::Blue,
            CardColor::Green => colored::Color::Green,
            CardColor::Red => colored::Color::Red,
            CardColor::White => colored::Color::White,
            CardColor::Yellow => colored::Color::Yellow,
        }
    }
}

impl From<CardColor> for char {
    fn from(value: CardColor) -> Self {
        match value {
            CardColor::Blue => 'x',
            CardColor::Green => '%',
            CardColor::Red => '+',
            CardColor::White => '@',
            CardColor::Yellow => '*',
        }
    }
}

impl Display for CardColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            char::from(*self)
                .to_string()
                .color(colored::Color::from(*self))
        )
    }
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

use crate::Render;
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
    pub color: CardColor,
    pub value: CardValue,
}

impl Render for Card {
    fn render(&self) -> String {
        [
            "┌────────┐\n",
            &format!("│ {}      │\n", self.value),
            &format!("│ {}      │\n", self.color),
            "│        │\n",
            "│        │\n",
            "│        │\n",
            "└────────┘",
        ]
        .into_iter()
        .collect()
    }
}

impl Card {
    pub fn new(color: CardColor, value: CardValue) -> Self {
        Self { color, value }
    }
}

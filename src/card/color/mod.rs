#[macro_export]
macro_rules! card_color {
    ("blue") => {
        CardColor::Blue
    };
    ("green") => {
        CardColor::Green
    };
    ("red") => {
        CardColor::Red
    };
    ("white") => {
        CardColor::White
    };
    ("yellow") => {
        CardColor::Yellow
    };
}

#[derive(Debug, Clone, Copy)]
pub enum CardColor {
    Blue,
    Green,
    Red,
    White,
    Yellow,
}

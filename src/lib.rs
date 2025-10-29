pub mod area;
pub mod card;
pub mod deck;
pub mod game;
pub mod hand;

pub trait Render {
    fn render(&self) -> String;
}

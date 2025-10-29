use crate::{card::color::CardColor, pile::scoring::ScoringPile};

#[allow(dead_code)]
pub struct ScoringArea([ScoringPile; 5]);

impl Default for ScoringArea {
    fn default() -> Self {
        Self([
            ScoringPile::new(None, CardColor::Blue),
            ScoringPile::new(None, CardColor::Green),
            ScoringPile::new(None, CardColor::Red),
            ScoringPile::new(None, CardColor::White),
            ScoringPile::new(None, CardColor::Yellow),
        ])
    }
}

use crate::{card::Card, deck::Deck};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Hand(Vec<Card>);

impl Hand {
    pub fn new(cards: Vec<Card>) -> Self {
        Hand(cards)
    }

    pub fn draw(deck: &mut Deck) -> Self {
        Self((0..5).filter_map(|_| deck.draw()).collect())
    }
}

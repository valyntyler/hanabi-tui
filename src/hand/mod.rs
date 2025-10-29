use crate::{card::Card, deck::Deck};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Hand(Vec<Card>);

impl Hand {
    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn new(cards: Vec<Card>) -> Self {
        Hand(cards)
    }

    pub fn draw(&mut self, deck: &mut Deck) {
        let mut rest = (self.len()..5)
            .filter_map(|_| deck.draw())
            .collect::<Vec<Card>>();
        self.0.append(&mut rest);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

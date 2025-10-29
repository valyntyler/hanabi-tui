use crate::{
    area::{discard::DiscardArea, scoring::ScoringArea},
    card::Card,
    deck::Deck,
};

#[derive(Debug)]
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

    pub fn play(&mut self, scoring: &mut ScoringArea, discard: &mut DiscardArea, index: usize) {
        if let Err(card) = scoring.play(self.0.remove(index)) {
            discard.discard(card)
        }
    }

    pub fn discard(&mut self, discard: &mut DiscardArea, index: usize) {
        discard.discard(self.0.remove(index));
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

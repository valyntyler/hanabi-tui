use crate::card::Card;

#[allow(dead_code)]
pub struct DiscardPile(Vec<Card>);

impl DiscardPile {
    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn discard(&mut self, card: Card) {
        self.0.push(card)
    }
}

use crate::card::Card;

#[derive(Debug)]
pub struct DiscardArea(Vec<Card>);

impl DiscardArea {
    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn discard(&mut self, card: Card) {
        self.0.push(card)
    }
}

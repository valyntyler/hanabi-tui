use crate::card::Card;

#[derive(Debug)]
pub struct Hand([Card; 5]);

impl Hand {
    pub fn new(cards: [Card; 5]) -> Self {
        Hand(cards)
    }
}

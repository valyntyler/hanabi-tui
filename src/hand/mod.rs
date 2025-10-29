use crate::card::Card;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Hand(Vec<Card>);

impl Hand {
    pub fn new(cards: Vec<Card>) -> Self {
        Hand(cards)
    }
}

use crate::card::Card;
use crate::card::color::CardColor;
use crate::card_value;

use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
pub struct Deck(Vec<Card>);

impl Default for Deck {
    fn default() -> Self {
        let mut deck = Self(
            CardColor::all()
                .map(|color| {
                    [
                        [Card::new(color, card_value!(1))].repeat(3),
                        [Card::new(color, card_value!(2))].repeat(2),
                        [Card::new(color, card_value!(3))].repeat(2),
                        [Card::new(color, card_value!(4))].repeat(2),
                        [Card::new(color, card_value!(5))].to_vec(),
                    ]
                    .into_iter()
                    .flatten()
                })
                .into_iter()
                .flatten()
                .collect(),
        );
        deck.shuffle();
        deck
    }
}

impl Deck {
    pub fn draw(&mut self) -> Option<Card> {
        self.0.pop()
    }

    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.0.shuffle(&mut rng);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

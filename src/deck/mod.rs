use crate::card::Card;
use crate::card::color::CardColor;
use crate::card_value;

#[derive(Debug)]
pub struct Deck(Vec<Card>);

impl Default for Deck {
    fn default() -> Self {
        Self(
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
        )
    }
}

impl Deck {
    pub fn draw(&mut self) -> Option<Card> {
        self.0.pop()
    }
}

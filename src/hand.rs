use crate::card_suit::Suit;
use crate::card_value::CardValue;
use crate::poker_combination::PokerCombination;
use crate::{card_suit, card_value};

#[derive(Copy, Clone, Debug)]
pub struct Card {
    pub suit: Suit,
    pub value: CardValue,
}

impl Card {
    pub fn random_new() -> Self {
        Card {
            suit: card_suit::Suit::generate_random(),
            value: card_value::CardValue::generate_random(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cards(Vec<Card>);

#[derive(Clone, Debug)]
pub struct Hand {
    cards: Cards,
}

impl Hand {
    pub fn new() -> Self {
        Hand::default()
    }

    pub fn put_card(&mut self, card: Card) {
        self.cards.0.push(card);
    }

    pub fn put_cards(&mut self, cards: &Hand) {
        for card in &cards.cards.0 {
            self.cards.0.push(*card);
        }
    }

    pub fn clear_cards(&mut self) {
        self.cards.0.clear();
    }

    pub fn len(&self) -> usize{
        self.cards.0.len()
    }
}

impl Default for Hand {
    fn default() -> Self {
        Hand {
            cards: Cards(vec![]),
        }
    }
}

impl Hand {
    pub fn get_combination(&self) -> PokerCombination {
        // implement discovering all the combinations from a collection of cards. For now, it is very basic and not fully working
        //                                                        v why is there a moved here?
        let mut values = self
            .cards
            .0
            .iter()
            .map(|x| x.value)
            .collect::<Vec<CardValue>>();
        let initial_size = values.len() as i128;
        values.sort();
        values.dedup();
        let size_after_removing_dupes = values.len() as i128;
        match (size_after_removing_dupes - initial_size).abs() {
            0 => PokerCombination::HighCard,
            1 => PokerCombination::Pair,
            2 => PokerCombination::Three,
            3 => PokerCombination::Quad,
            _ => PokerCombination::HighCard,
        }
    }

    pub fn is_matching(&self, right: &PokerCombination) -> bool {
        self.get_combination() == *right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quad_does_not_match_pair() {
        let pair_hand = Hand {
            cards: Cards(vec![
                Card {
                    suit: Suit::Diamonds,
                    value: CardValue::Two,
                },
                Card {
                    suit: Suit::Clubs,
                    value: CardValue::Two,
                },
                Card {
                    suit: Suit::Hearts,
                    value: CardValue::Three,
                },
                Card {
                    suit: Suit::Spades,
                    value: CardValue::Four,
                },
                Card {
                    suit: Suit::Diamonds,
                    value: CardValue::Five,
                },
            ]),
        };
        assert!(!pair_hand.is_matching(&PokerCombination::Quad));
    }

    #[test]
    fn quad_matches_quad() {
        let quad_hand = Hand {
            cards: Cards(vec![
                Card {
                    suit: Suit::Diamonds,
                    value: CardValue::Two,
                },
                Card {
                    suit: Suit::Clubs,
                    value: CardValue::Two,
                },
                Card {
                    suit: Suit::Hearts,
                    value: CardValue::Two,
                },
                Card {
                    suit: Suit::Spades,
                    value: CardValue::Two,
                },
                Card {
                    suit: Suit::Diamonds,
                    value: CardValue::Five,
                },
            ]),
        };
        assert!(quad_hand.is_matching(&PokerCombination::Quad));
    }
}

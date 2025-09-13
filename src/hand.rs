use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};

use crate::card_suit::Suit;
use crate::card_value::CardValue;
use crate::poker_combination::PokerCombination;

// This whole file is to refactor
#[derive(Debug)]
pub enum DeckError {
    EmptyDeck,
}

#[derive(Clone, Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        let values = [
            CardValue::Nine,
            CardValue::Ten,
            CardValue::Jack,
            CardValue::Queen,
            CardValue::King,
            CardValue::Ace,
        ];
        for suit in suits {
            for value in values {
                cards.push(Card::new(suit, value));
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
    pub fn draw(&mut self) -> Result<Card, DeckError> {
        if !self.cards.is_empty() {
            let card = self.cards.pop().unwrap();
            Ok(card)
        } else {
            println!("Cannot draw from mi");
            Err(DeckError::EmptyDeck)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub value: CardValue,
}

impl Card {
    pub fn new(suit: Suit, value: CardValue) -> Self {
        Card {
            suit,
            value,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Hand::default()
    }

    pub fn put_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn put_cards(&mut self, cards: &Hand) {
        for card in &cards.cards {
            self.cards.push(*card);
        }
    }

    pub fn clear_cards(&mut self) {
        self.cards.clear();
    }

    #[allow(dead_code)] // Used in UT for now
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn discover_combinations(&self) -> Vec<PokerCombination> {
        let mut combinations_found = Vec::new();
        self.find_flush(&mut combinations_found);
        self.find_quantity_figures(&mut combinations_found);
        self.find_straight(&mut combinations_found);
        // The method below relies on methods above being called before to discover straight and flush. Probably code structure issue
        self.is_straight_or_royal_flush(&mut combinations_found);
        combinations_found
    }

    fn find_flush(&self, combinations_found: &mut Vec<PokerCombination>) {
        let is_flush = self
            .cards
            .iter()
            .map(|x| x.suit)
            .collect::<HashSet<Suit>>()
            .len()
            == 1;
        if is_flush {
            combinations_found.push(PokerCombination::Flush);
        }
    }

    fn find_quantity_figures(&self, combinations_found: &mut Vec<PokerCombination>) {
        let value_count_map = self.get_value_count_map();
        let number_of_doubles = value_count_map
            .iter()
            .filter(|&value_count_pair| value_count_pair.1 >= &2u8)
            .count();
        match number_of_doubles {
            1 => {
                combinations_found.push(PokerCombination::Pair);
            }
            2 => {
                combinations_found.push(PokerCombination::Pair);
                combinations_found.push(PokerCombination::TwoPairs);
            }
            _ => {}
        }
        if value_count_map.values().any(|&x| x >= 3) {
            combinations_found.push(PokerCombination::Three);
        }
        if value_count_map.values().any(|&x| x >= 4) {
            // Four of a kind does not contain two pairs according to the Internet
            combinations_found.push(PokerCombination::Quad);
        }
        let number_of_gt_than_doubles = value_count_map
            .iter()
            .filter(|&value_count_pair| value_count_pair.1 > &2u8)
            .count();
        if (number_of_doubles > 0 && number_of_gt_than_doubles > 0) || number_of_gt_than_doubles > 1
        {
            combinations_found.push(PokerCombination::FullHouse);
        }
    }

    fn get_value_count_map(&self) -> HashMap<CardValue, u8> {
        let values = self
            .cards
            .iter()
            .map(|x| x.value)
            .collect::<Vec<CardValue>>();
        let mut value_count_map: HashMap<CardValue, u8> = HashMap::new();
        for value in values {
            *value_count_map.entry(value).or_insert(0) += 1;
        }
        value_count_map
    }

    fn find_straight(&self, combinations_found: &mut Vec<PokerCombination>) {
        let mut vals = self
            .cards
            .iter()
            .map(|x| x.value)
            .collect::<Vec<CardValue>>();
        vals.sort();
        for (i, _) in vals.iter().enumerate() {
            if vals.len() - i < 5 {
                return;
            }
            if self.is_given_cards_straight(&vals[i..i + 5]) {
                combinations_found.push(PokerCombination::Straight);
            }
        }
    }

    fn is_given_cards_straight(&self, values: &[CardValue]) -> bool {
        assert!(
            values.is_sorted(),
            "The slice has to be sorted in order to discover straight"
        );
        if values.len() == 5 {
            let mut current = values.first().expect("No values in given cards vector");
            for remaining in &values[1..] {
                if (*remaining as u8) - (*current as u8) != 1 {
                    return false;
                }
                current = remaining
            }
            return true;
        }
        false
    }

    fn is_straight_or_royal_flush(&self, combinations_found: &mut Vec<PokerCombination>) {
        if combinations_found.contains(&PokerCombination::Flush)
            && combinations_found.contains(&PokerCombination::Straight)
        {
            if self.cards.iter().any(|x| x.value == CardValue::Ace)
                && self.cards.iter().any(|x| x.value == CardValue::Ten)
            {
                combinations_found.push(PokerCombination::RoyalFlush);
            } else {
                combinations_found.push(PokerCombination::StraightFlush);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_quad_hand() -> Hand {
        let mut hand = Hand {
            cards: vec![
                Card {
                    suit: Suit::Diamonds,
                    value: CardValue::Two,
                };
                4
            ],
        };
        hand.put_card(Card {
            suit: Suit::Diamonds,
            value: CardValue::Five,
        });
        hand
    }

    fn get_fullhouse_hand() -> Hand {
        Hand {
            cards: vec![
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
                    value: CardValue::Three,
                },
                Card {
                    suit: Suit::Diamonds,
                    value: CardValue::Three,
                },
            ],
        }
    }

    fn get_two_pairs_hand() -> Hand {
        Hand {
            cards: vec![
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
                    value: CardValue::Three,
                },
                Card {
                    suit: Suit::Diamonds,
                    value: CardValue::Five,
                },
            ],
        }
    }

    fn get_straight_flush_hand() -> Hand {
        Hand {
            cards: vec![
                Card {
                    suit: Suit::Diamonds,
                    value: CardValue::Six,
                },
                Card {
                    suit: Suit::Diamonds,
                    value: CardValue::Six,
                },
                Card {
                    suit: Suit::Diamonds,
                    value: CardValue::Seven,
                },
                Card {
                    suit: Suit::Diamonds,
                    value: CardValue::Eight,
                },
                Card {
                    suit: Suit::Diamonds,
                    value: CardValue::Nine,
                },
                Card {
                    suit: Suit::Diamonds,
                    value: CardValue::Ten,
                },
                Card {
                    suit: Suit::Diamonds,
                    value: CardValue::Six,
                },
            ],
        }
    }

    fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }
    #[test]
    fn creates_new_hand_empty() {
        let new_hand = Hand::new();
        assert!(new_hand.cards.is_empty());
    }

    #[test]
    fn can_put_cards_into_existing_hand() {
        let mut new_hand = Hand::new();
        assert!(new_hand.cards.is_empty());
        new_hand.put_card(Card::new(
            Suit::generate_random(),
            CardValue::generate_random(),
        ));
        assert_eq!(new_hand.cards.len(), 1);
        new_hand.put_cards(&get_two_pairs_hand());
        assert_eq!(new_hand.cards.len(), 6);
    }

    #[test]
    fn can_clear_cards() {
        let mut new_hand = Hand::new();
        assert!(new_hand.cards.is_empty());
        new_hand.put_card(Card::new(
            Suit::generate_random(),
            CardValue::generate_random(),
        ));
        assert_eq!(new_hand.cards.len(), 1);
        new_hand.clear_cards();
        assert_eq!(new_hand.cards.len(), 0);
    }

    #[test]
    fn get_combination_handles_pair_twopairs_three_quad() {
        assert!(
            get_quad_hand()
                .discover_combinations()
                .contains(&PokerCombination::Quad)
        );
        assert!(
            get_quad_hand()
                .discover_combinations()
                .contains(&PokerCombination::Pair)
        );
        assert!(
            get_two_pairs_hand()
                .discover_combinations()
                .contains(&PokerCombination::TwoPairs)
        );
        assert!(
            get_quad_hand()
                .discover_combinations()
                .contains(&PokerCombination::Three)
        );
    }

    #[test]
    fn get_combination_quad_is_not_two_pairs() {
        assert!(
            !get_quad_hand()
                .discover_combinations()
                .contains(&PokerCombination::TwoPairs)
        );
    }

    #[test]
    fn get_combination_handles_fullhouse() {
        assert!(
            get_fullhouse_hand()
                .discover_combinations()
                .contains(&PokerCombination::FullHouse)
        );
    }

    #[test]
    fn get_combination_handles_flush() {
        assert!(
            get_quad_hand()
                .discover_combinations()
                .contains(&PokerCombination::Flush)
        );
    }

    #[test]
    fn get_combination_handles_straight_in_the_middle_of_hand() {
        assert!(
            get_straight_flush_hand()
                .discover_combinations()
                .contains(&PokerCombination::Straight)
        );
    }

    #[test]
    fn get_combination_handles_straight_flush_in_the_middle_of_hand() {
        assert!(
            get_straight_flush_hand()
                .discover_combinations()
                .contains(&PokerCombination::StraightFlush)
        );
    }
    #[test]
    fn test_basic_deck() {
        let mut sut_deck = Deck::new();
        let mut cards = Vec::new();
        for _ in 0..24 {
            cards.push(
                sut_deck
                    .draw()
                    .expect("Draw is expected to be possible 24 times!"),
            );
        }
        assert!(do_vecs_match(
            &cards,
            &vec![
                Card::new(Suit::Spades, CardValue::Ace),
                Card::new(Suit::Spades, CardValue::King),
                Card::new(Suit::Spades, CardValue::Queen),
                Card::new(Suit::Spades, CardValue::Jack),
                Card::new(Suit::Spades, CardValue::Ten),
                Card::new(Suit::Spades, CardValue::Nine),
                Card::new(Suit::Hearts, CardValue::Ace),
                Card::new(Suit::Hearts, CardValue::King),
                Card::new(Suit::Hearts, CardValue::Queen),
                Card::new(Suit::Hearts, CardValue::Jack),
                Card::new(Suit::Hearts, CardValue::Ten),
                Card::new(Suit::Hearts, CardValue::Nine),
                Card::new(Suit::Diamonds, CardValue::Ace),
                Card::new(Suit::Diamonds, CardValue::King),
                Card::new(Suit::Diamonds, CardValue::Queen),
                Card::new(Suit::Diamonds, CardValue::Jack),
                Card::new(Suit::Diamonds, CardValue::Ten),
                Card::new(Suit::Diamonds, CardValue::Nine),
                Card::new(Suit::Clubs, CardValue::Ace),
                Card::new(Suit::Clubs, CardValue::King),
                Card::new(Suit::Clubs, CardValue::Queen),
                Card::new(Suit::Clubs, CardValue::Jack),
                Card::new(Suit::Clubs, CardValue::Ten),
                Card::new(Suit::Clubs, CardValue::Nine),
            ]
        ));
    }
    #[test]
    fn test_empty_deck_behaviour() {
        let mut sut_deck = Deck::new();
        for _ in 0..24 {
            sut_deck
                .draw()
                .expect("Draw is expected to be possible 24 times!");
        }
        sut_deck.draw().expect_err("Expected Error on 25th draw");
    }
}

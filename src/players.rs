use crate::hand::{Card, Deck, Hand};

#[derive(Clone)]
pub struct Player {
    hand: Hand,
    number_of_cards_to_deal: u8,
}

impl Player {
    fn new(hand: Hand) -> Self {
        Self {
            hand,
            number_of_cards_to_deal: 1,
        }
    }
    fn put_card(&mut self, card: Card) {
        self.hand.put_card(card)
    }
    pub fn print_hand(&self) {
        println!("{:?}", self.hand)
    }
    pub fn hand(&self) -> &Hand {
        &self.hand
    }

    pub fn increase_number_of_cards_to_deal(&mut self) {
        self.number_of_cards_to_deal += 1;
    }
}

pub struct Players {
    players: Vec<Player>,
}

impl Players {
    pub fn new(no_of_players: usize) -> Self {
        let players = vec![Player::new(Hand::new()); no_of_players];
        let mut players = Players { players };
        println!("Dealing cards");
        players.deal_cards();
        println!("Cards dealt");
        players
    }

    pub fn deal_cards(&mut self) {
        let mut deck = Deck::new();
        deck.shuffle();
        for player in self.players.iter_mut() {
            for _ in 0..player.number_of_cards_to_deal {
                match deck.draw() {
                    Ok(card) => {
                        player.put_card(card);
                    }
                    Err(_) => {
                        panic!("Critical problem with deck - not implemented handling!")
                    }
                }
            }
        }
    }
    pub fn get(&self) -> &Vec<Player> {
        &self.players
    }

    #[allow(dead_code)] // Used in UT for now
    pub fn len(&self) -> usize {
        self.players.len()
    }

    pub fn get_mut(&mut self) -> &mut Vec<Player> {
        &mut self.players
    }

    pub fn empty_all_cards(&mut self) {
        for player in self.players.iter_mut() {
            player.hand.clear_cards();
        }
    }

    pub fn get_all_cards(&self) -> Hand {
        let mut all_cards = Hand::new();
        for player in self.get() {
            all_cards.put_cards(player.hand());
        }
        println!("All cards: {:?}", all_cards);
        all_cards
    }

    pub fn is_limit_hit(&self, limit: u8) -> bool {
        self.get()
            .iter()
            .any(|player| player.number_of_cards_to_deal == limit)
    }
}

#[cfg(test)]
mod tests {
    use crate::card_suit::Suit;
    use crate::card_value::CardValue;
    use crate::hand::Card;
    use crate::players::Players;

    #[test]
    fn creates_new_players_with_given_count() {
        let players = Players::new(3);
        assert_eq!(players.len(), 3)
    }

    #[test]
    fn empties_all_cards_in_all_hands() {
        let mut players = Players::new(3);
        assert_eq!(players.get_all_cards().len(), 3);
        players.empty_all_cards();
        assert_eq!(players.get_all_cards().len(), 0);
    }

    #[test]
    fn when_one_player_reaches_card_limit_is_limit_hit_returns_true() {
        let players = Players::new(3);
        assert!(players.is_limit_hit(1));
        assert!(!players.is_limit_hit(2));
    }

    #[test]
    fn deal_card_deals_appropriately_to_each_player() {
        let mut players = Players::new(3);
        players.get_mut()[0].number_of_cards_to_deal = 3;
        players.empty_all_cards();
        players.deal_cards();
        assert_eq!(players.get()[0].hand.len(), 3);
        assert_eq!(players.get()[1].hand.len(), 1);
        assert_eq!(players.get()[2].hand.len(), 1);
    }
    #[test]
    fn increase_number_of_cards_to_deal_works() {
        let mut players = Players::new(1);
        players.get_mut()[0].increase_number_of_cards_to_deal();
        assert_eq!(players.get()[0].number_of_cards_to_deal, 2);
    }

    #[test]
    fn put_card_works() {
        let mut players = Players::new(1);
        players.get_mut()[0].put_card(Card::new(Suit::Clubs, CardValue::Ace));
        assert_eq!(players.get_all_cards().len(), 2); // 1 on creation and another one on put_card call
    }
}

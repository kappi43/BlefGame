use crate::hand::{Card, Hand};

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
    players: Vec<Player>
}

impl Players {
    pub fn new(no_of_players: u8) -> Self {
        let players = vec![Player::new(Hand::new()); no_of_players as usize];
        let mut players = Players { players };
        println!("Dealing cards");
        players.deal_cards();
        println!("Cards dealt");
        players
    }

    pub fn deal_cards(&mut self) {
        for player in self.players.iter_mut() {
            for _ in 0..player.number_of_cards_to_deal {
                let card = Card::random_new();
                player.put_card(card)
            }
        }
    }
    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn players_mut(&mut self) -> &mut Vec<Player> {
        &mut self.players
    }

    pub fn empty_all_cards(&mut self) {
        for player in self.players.iter_mut() {
            player.hand.clear_cards();
        }
    }

    pub fn get_all_cards(&self) -> Hand {
        let mut all_cards = Hand::new();
        for player in self.players() {
            all_cards.put_cards(player.hand());
            println!("{:?}", all_cards);
        }
        all_cards
    }

    pub fn is_limit_hit(&self, limit: u8) -> bool {
        self.players().iter().any(|player|player.number_of_cards_to_deal == limit)
    }
}

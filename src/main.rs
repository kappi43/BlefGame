use std::io;

use commands::Commands;
use poker_combination::PokerCombination;

use crate::hand::Hand;
use crate::players::Players;

mod card_suit;
mod card_value;
mod hand;
mod players;
mod poker_combination;
mod utils;
mod commands;

pub struct Config{
    no_of_players: u8,
    card_on_hand_limit: u8,
}
impl Config{
    pub fn get_config() -> Self{
        println!("Please input the configuration for the game");
        println!("number of players:");
        let no_of_players:u8 = text_io::try_read!().expect("Did not enter a valid u8");
        println!("cards on hands limit:");
        let card_on_hand_limit:u8 = text_io::try_read!().expect("Did not enter a valid u8");
        Config{no_of_players, card_on_hand_limit}
    }
}

fn main() {
    println!("Welcome to bluff!");
    let config: Config = Config::get_config();
    let mut players = Players::new(config.no_of_players);
    let mut current_bet = PokerCombination::None;
    while !players.is_limit_hit(config.card_on_hand_limit) {
        play_round(&mut players, &mut current_bet);
    }
    println!("Game over. A player reached the card limit. Press ENTER to continue");

    io::stdin()
        .read_line(&mut String::new())
        .expect("Failed in reading user input");
}


fn play_round(players: &mut Players, current_bet: &mut PokerCombination) {
    println!("Beginning new round");

    let all_cards = players.get_all_cards();
    for (index, player) in players.players_mut().iter_mut().enumerate() {
        println!("Current bet: {:?}", current_bet);
        println!("Player {index}");
        player.print_hand();
        let mut command = commands::get_next_command();
        while command == Commands::Unknown{
            command = commands::get_next_command();
        }
        match command {
            Commands::Bet(value) => {
                handle_new_bet(value, current_bet);
            }
            Commands::Call => {
                let result = check_round_result(current_bet, &all_cards);
                if result {
                    player.increase_number_of_cards_to_deal();
                } else if index == 0 {
                    players
                        .players_mut()
                        .last_mut()
                        .unwrap()
                        .increase_number_of_cards_to_deal();
                } else {
                    players
                        .players_mut()
                        .get_mut(index - 1)
                        .unwrap()
                        .increase_number_of_cards_to_deal()
                }
                players.empty_all_cards();
                players.deal_cards();
                *current_bet = PokerCombination::None;
                return;
            }
            Commands::Unknown => {}
        }
        utils::clear_screen();
    }
}

fn check_round_result(current_bet: &PokerCombination, all_cards: &Hand) -> bool {
    all_cards.is_matching(current_bet) // to change into "contains combination"
}

fn handle_new_bet(new_bet: PokerCombination, current_bet: &mut PokerCombination) {
    if new_bet <= *current_bet {
        println!("The new bet has to be bigger than the existing one");
    } else {
        *current_bet = new_bet;
    }
}

fn get_bet(bet_str: &str) -> Result<PokerCombination, String> {
    PokerCombination::try_from(bet_str)
}

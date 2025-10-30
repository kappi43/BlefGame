use std::io;

use commands::Commands;
use config::Config;
use poker_combination::PokerCombination;

use crate::hand::Hand;
use crate::players::Players;

mod card_suit;
mod card_value;
mod commands;
mod config;
mod hand;
mod players;
mod poker_combination;
mod utils;

fn main() {
    // Most of this code should be in GameLogic
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

// To refactor and extract all the below functions to GameLogic module
fn play_round(players: &mut Players, current_bet: &mut PokerCombination) {
    println!("Beginning new round");

    let all_cards = players.get_all_cards();
    for (index, player) in players.get_mut().iter_mut().enumerate() {
        println!("Current bet: {:?}", current_bet);
        println!("Player {index}");
        player.print_hand();
        // Move the below command getting loop into a method in commands? try_get_next_command_until_success?
        let mut command = commands::get_next_command();
        while command == Commands::Unknown {
            command = commands::get_next_command();
        }
        match command {
            Commands::Bet(value) => {
                handle_new_bet(value, current_bet);
            }
            Commands::Call => {
                let result = check_round_result(current_bet, &all_cards); // Can return Result<RoundResult> to function above and handle round end there. This would save
                //MAYBE players could be linked list. This would clean up this bit below A LOT. We don't care that much about performance, we probably will have at most close to 10 elements in the data structure.
                if result {
                    player.increase_number_of_cards_to_deal();
                } else if index == 0 {
                    players
                        .get_mut()
                        .last_mut()
                        .unwrap()
                        .increase_number_of_cards_to_deal();
                } else {
                    players
                        .get_mut()
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
    all_cards.discover_combinations().contains(current_bet)
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

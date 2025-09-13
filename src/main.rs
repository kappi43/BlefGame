use std::io;
use std::ops::Index;
use commands::Commands;
use config::Config;
use poker_combination::PokerCombination;

use crate::hand::Hand;
use crate::players::{Player, Players};

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
    println!("All cards: {:?}", players.get_all_cards());
    for i in 0..players.len(){
            println!("Current bet: {:?}", current_bet);
            println!("Player {i}");
            players.players()[i].print_hand();
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
                    handle_call(players, current_bet,i);
                    return;
                }
                Commands::Unknown => {}
            }
            utils::clear_screen();
    }
}

fn handle_call(players: &mut Players, current_bet: &mut PokerCombination, index: usize) {
    let result = did_call_succeed(current_bet, &players.get_all_cards());
    handle_round_result(players, index, result);
    reset_game_state(players, current_bet);
}

fn handle_round_result(players: &mut Players, index: usize, result: bool) {
    if result {
        players.players_mut()[index].increase_number_of_cards_to_deal();
    } else {
        increase_previous_players_number_of_cards_to_deal(players, index);
    }
}

fn increase_previous_players_number_of_cards_to_deal(players: &mut Players, index: usize) {
    let len = players.len();
    let previous_index = (index + len - 1) % len;
    players
        .players_mut()
        .get_mut(previous_index)
        .unwrap()
        .increase_number_of_cards_to_deal();
}

fn reset_game_state(players: &mut Players, current_bet: &mut PokerCombination) {
    players.empty_all_cards();
    players.deal_cards();
    *current_bet = PokerCombination::None;
}

fn did_call_succeed(current_bet: &PokerCombination, all_cards: &Hand) -> bool {
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

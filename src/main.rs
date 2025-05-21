use std::io;
use poker_combination::PokerCombination;

use crate::hand::Hand;
use crate::players::Players;

//use std::io::Write;

mod card_suit;
mod card_value;
mod hand;
mod players;
mod poker_combination;
mod utils;
mod commands;

use commands::Commands;


fn main() {
    println!("Welcome to bluff!");
    let mut players = Players::new(3);
    let mut current_bet = PokerCombination::None;
    while !players.is_limit_hit(6) {
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

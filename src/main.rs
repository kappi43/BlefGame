use std::io;

use clearscreen::ClearScreen;

use poker_combination::PokerCombination;

use crate::hand::Hand;
use crate::players::Players;

//use std::io::Write;

mod card_suit;
mod card_value;
mod hand;
mod players;
mod poker_combination;

#[derive(PartialEq)]
enum Commands {
    Bet(PokerCombination),
    Call,
    Unknown,
}

fn main() {
    println!("Welcome to bluff!");
    let mut players = Players::new(3);
    let mut current_bet = PokerCombination::None;
    loop {
        play_round(&mut players, &mut current_bet);
    }
}

fn play_round(players: &mut Players, current_bet: &mut PokerCombination) {
    println!("Beginning new round");
    /**************/
 // Dirty hack
    let mut all_cards = players.get_all_cards();
    /**************/
    for (index, player) in players.players_mut().iter_mut().enumerate() {
        println!("Current bet: {:?}", current_bet);
        println!("Player {index}");
        player.print_hand();
        let mut command = get_next_command();
        while command == Commands::Unknown{
            command = get_next_command();
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
        clear_screen();
    }
}

fn check_round_result(current_bet: &PokerCombination, all_cards: &Hand) -> bool {
    all_cards.is_matching(current_bet) // to change into "contains combination"
}

fn clear_screen() {
    let res = ClearScreen::default().clear();
    if let Err(e) = res {
        println!("Could not clear screen: {e}");
    }
}

fn get_next_command() -> Commands {
    let mut input = String::new();
    println!("Please input command");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed in reading user input");
    if input.to_lowercase().starts_with("bet") {
        let new_bet = get_bet(&input);
        Commands::Bet(new_bet)
    } else if input.to_lowercase().starts_with("call") {
        Commands::Call
    } else {
        println!("please use one of those commands: bet, call");
        Commands::Unknown
    }
}

fn handle_new_bet(new_bet: PokerCombination, current_bet: &mut PokerCombination) {
    if new_bet <= *current_bet {
        println!("The new bet has to be bigger than the existing one");
    } else {
        *current_bet = new_bet;
    }
}

fn get_bet(bet_str: &str) -> Result<PokerCombination, String> {
    // To improve the error handling in this function. The command format error is easily recoverable
    let split = bet_str.split(" ").collect::<Vec<&str>>();
    assert_eq!(split.len(), 2, "Incorrect number of arguments to \"bet\" command. Expected: bet <name of proposed combination>");
    assert_eq!(
        split[0].to_lowercase(),
        "bet",
        "{} {}",
        "Unknown command. Expected: bet, Actual: ",
        split[0]
    );
    let combination = PokerCombination::try_from(split[1]);
    // combination.unwrap_or_else(|e| {
    //     panic!("{}", e)
    // })
    combination
}

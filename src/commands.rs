use std::io;
use crate::poker_combination::PokerCombination;

#[derive(PartialEq)]
pub enum Commands {
    Bet(PokerCombination),
    Call,
    Unknown,
}

pub fn get_next_command() -> Commands {
    let mut input = String::new();
    println!("Please input command");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed in reading user input");
    let split = input.trim().trim_end().split(" ").collect::<Vec<&str>>();
    if split.len() < 1{
        println!("Invalid command format. Expected: bet <POKER COMBINATION>|call");
        return Commands::Unknown
    }
    match split[0] {
        "bet" => {
            if split.len() != 2 {
                println!("Invalid command format. Expected: bet <POKER COMBINATION>|call");
                return Commands::Unknown
            }
            let new_bet = crate::get_bet(&split[1]);
            match new_bet {
                Ok(bet) => {Commands::Bet(bet)}
                Err(err) => {
                    println!("{}", err);
                    Commands::Unknown
                }
            }
        },
        "call" => {
            if split.len() != 1 {
                println!("Invalid command format. Expected: bet <POKER COMBINATION>|call");
                return Commands::Unknown
            }
            Commands::Call
        },
        _ => {
            println!("please use one of those commands: bet, call");
            Commands::Unknown
        }
    }
}

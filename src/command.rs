use std::io;
use std::str::FromStr;
use crate::poker_combination::PokerCombination;

#[derive(PartialEq, Debug)]
pub enum Command {
    Bet(PokerCombination),
    Call,
}

pub fn get_next_command() -> Result<Command, ()> {
    let mut input = String::new();
    println!("Please input command");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed in reading user input");
    println!("Command: {input}");
    Command::from_str(input.as_str())
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.trim().trim_end().split(" ").collect::<Vec<&str>>();
        if split.is_empty() {
            println!("Invalid command format. Expected: bet <POKER COMBINATION>|call");
            return Err(());
        }
        match split[0] {
            "bet" => {
                if split.len() != 2 {
                    println!("Invalid command format. Expected: bet <POKER COMBINATION>|call");
                    return Err(());
                }
                let new_bet = crate::get_bet(split[1]);
                match new_bet {
                    Ok(bet) => Ok(Command::Bet(bet)),
                    Err(err) => {
                        println!("{}", err);
                        Err(())
                    }
                }
            }
            "call" => {
                if split.len() != 1 {
                    println!("Invalid command format. Expected: bet <POKER COMBINATION>|call");
                    return Err(());
                }
                Ok(Command::Call)
            }
            _ => {
                println!("Please use one of those commands: bet, call");
                Err(())
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn command_correctly_parses_valid_bet_command() {
        let valid_bet_command = "bet pair";
        assert_eq!(Ok(Command::Bet(PokerCombination::Pair)), Command::from_str(valid_bet_command));
    }
    #[test]
    fn command_fails_to_parse_invalid_bet_command() {
        let invalid_bet_command = "bet";
        assert!(Command::from_str(invalid_bet_command).is_err());

        let invalid_bet_command = "bet pa";
        assert!(Command::from_str(invalid_bet_command).is_err());
    }

    #[test]
    fn command_fails_to_parse_invalid_command() {
        let invalid_bet_command = "make coffee";
        assert!(Command::from_str(invalid_bet_command).is_err());
    }

    #[test]
    fn command_correctly_parses_valid_call_command() {
        let valid_call_command = "call";
        assert_eq!(Ok(Command::Call), Command::from_str(valid_call_command));
    }

    #[test]
    fn command_fails_to_parse_invalid_call_command() {
        let invalid_call_command = "call something";
        assert!(Command::from_str(invalid_call_command).is_err());
    }
}

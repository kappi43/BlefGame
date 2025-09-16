use std::fmt::Debug;
use std::str::FromStr;

pub struct Config {
    pub(crate) no_of_players: usize,
    pub(crate) card_on_hand_limit: u8,
}

impl Config {
    pub fn get_config() -> Self {
        // to implement error handling
        println!("Please input the configuration for the game");
        println!("number of players:");
        let no_of_players: usize = Self::read_value_until_success("Please enter a valid number");
        println!("cards on hands limit:");
        let card_on_hand_limit: u8 = Self::read_value_until_success("Please enter a valid number");
        std::io::stdin() // This is a workaround as this library leaves newspaces behind and the newspaces are getting read on stdin::read_line call
            .read_line(&mut String::new())
            .expect("Could not clear buffer");
        Config {
            no_of_players,
            card_on_hand_limit,
        }
    }
    fn read_value_until_success<Type>(error_message: &str) -> Type
    where
        Type: FromStr + Debug,
        <Type as FromStr>::Err: Debug,
    {
        let mut value: Result<Type, _> = text_io::try_read!();
        while value.is_err() {
            println!("{}", error_message);
            value = text_io::try_read!();
        }
        value.unwrap()
    }
}

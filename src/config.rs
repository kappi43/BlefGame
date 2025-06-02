use std::io::Write;

pub struct Config {
    pub(crate) no_of_players: u8,
    pub(crate) card_on_hand_limit: u8,
}

impl Config {
    pub fn get_config() -> Self {
        // to implement error handling
        println!("Please input the configuration for the game");
        println!("number of players:");
        let no_of_players: u8 = text_io::try_read!().expect("Did not enter a valid u8");
        println!("cards on hands limit:");
        let card_on_hand_limit: u8 = text_io::try_read!().expect("Did not enter a valid u8");
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Could not clear buffer");
        Config {
            no_of_players,
            card_on_hand_limit,
        }
    }
}

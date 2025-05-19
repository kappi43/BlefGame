#[derive(PartialEq, PartialOrd, Debug)]
pub enum PokerCombination {
    None,
    HighCard,
    Pair,
    TwoPairs,
    Three,
    FullHouse,
    Quad,
    Straight,
    Flush,
    Poker,
    RoyalPoker,
}

impl TryFrom<&str> for PokerCombination {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().trim_end().to_lowercase().as_str() {
            "none" => Ok(PokerCombination::None),
            "highcard" => Ok(PokerCombination::HighCard),
            "pair" => Ok(PokerCombination::Pair),
            "twopairs" => Ok(PokerCombination::TwoPairs),
            "three" => Ok(PokerCombination::Three),
            "fullhouse" => Ok(PokerCombination::FullHouse),
            "quad" => Ok(PokerCombination::Quad),
            "straight" => Ok(PokerCombination::Straight),
            "flush" => Ok(PokerCombination::Flush),
            "poker" => Ok(PokerCombination::Poker),
            "royalpoker" => Ok(PokerCombination::RoyalPoker),
            _ => Err("Invalid PokerCombination name".to_string()),
        }
    }
}

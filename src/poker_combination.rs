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
    StraightFlush,
    RoyalFlush,
}

impl TryFrom<&str> for PokerCombination {
    // change into an actual error type
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
            "straightflush" => Ok(PokerCombination::StraightFlush),
            "royalflush" => Ok(PokerCombination::RoyalFlush),
            _ => Err("Invalid PokerCombination name".to_string()),
        }
    }
}

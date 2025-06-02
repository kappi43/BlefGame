use rand_derive2::RandGen;

#[derive(PartialEq, Ord, Eq, PartialOrd, Clone, Copy, RandGen, Debug, Hash)]
pub enum CardValue {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

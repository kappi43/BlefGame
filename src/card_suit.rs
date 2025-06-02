use rand_derive2::RandGen;

#[derive(Clone, Copy, RandGen, Debug, Eq, PartialEq, Hash)]
pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

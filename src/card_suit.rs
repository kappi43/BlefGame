use rand_derive2::RandGen;
#[derive(Clone, Copy, RandGen, Debug)]
pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

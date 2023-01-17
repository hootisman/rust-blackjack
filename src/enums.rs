

use strum_macros::EnumIter;

#[derive(Debug,EnumIter,Clone, Copy)]
pub enum Suits {
    Spade,
    Heart,
    Diamond,
    Club,
}
impl Suits {
    pub fn value(&self) -> char{
        match self {
            Self::Spade => '♠',
            Self::Heart => '♥',
            Self::Diamond => '♦',
            Self::Club => '♣',
        }
    }
}
#[derive(Debug,EnumIter,Clone, Copy)]
pub enum CardValues {
    Ace,
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
}
impl CardValues {
    pub fn value(&self) -> &str{
        match self {
            Self::Ace => "Ace",
            Self::Two => "Two",
            Self::Three => "Three",
            Self::Four => "Four",
            Self::Five => "Five",
            Self::Six => "Six",
            Self::Seven => "Seven",
            Self::Eight => "Eight",
            Self::Nine => "Nine",
            Self::Ten => "Ten",
            Self::Jack => "Jack",
            Self::Queen => "Queen",
            Self::King => "King",
        }
    }
}

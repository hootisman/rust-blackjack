
use std::fmt::{self, Display};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug,EnumIter,Clone, Copy)]
pub enum Suits {
    Spade,
    Heart,
    Diamond,
    Club,
}
impl Suits {
    fn value(&self) -> char{
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
    fn value(&self) -> &str{
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
pub struct Card {
    pub value: CardValues,
    pub suit: Suits,
}
impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}'s", self.value.value(),self.suit.value() )
    }
}

pub struct Deck {
    pub cards: Vec<Card>, 
}

impl Deck {
   pub fn new_default() -> Deck{
        //creates 52 card deck in standard order
        let mut cards = Vec::new();
        for new_suit in Suits::iter() {
            for new_value in CardValues::iter() {
                let card = Card { value: new_value, suit: new_suit };
                println!("{}",card);
                cards.push(card);
            }
        }

        Deck {
            cards,
        }
    }
}



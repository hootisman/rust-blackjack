use std::fmt::{self, Display};


enum Suits {
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
enum CardValues {
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
struct Card {
    value: CardValues,
    suit: Suits,
}
impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}'s", self.value.value(),self.suit.value() )
    }
}

fn main() {
    let spade: Suits = Suits::Spade;
    let diam: Suits = Suits::Diamond;
    let ace = CardValues::Ace;
    let card = Card { value: CardValues::King, suit: Suits::Heart };
    println!("{}",spade.value());
    println!("{}",ace.value());
    println!("{}",card.to_string());
}

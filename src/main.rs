
mod deck;
pub mod enums;

use crate::deck::{
    Card,
    Deck,
};
use crate::enums::{
    Suits,
    CardValues,
};

fn main() {
    let king: Card = Card { value: CardValues::Ace, suit: Suits::Spade }; 
    println!("{}",king); 
    let mut thedeck: Deck = Deck::new_default(); 

    thedeck.shuffle();
    thedeck.print();
}

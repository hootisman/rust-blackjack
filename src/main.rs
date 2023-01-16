use crate::deck::{
    Card,
    CardValues,
    Suits,
    Deck,
};

mod deck;
fn main() {
    let king: Card = Card { value: CardValues::Ace, suit: Suits::Spade }; 
    println!("{}",king); 
    let mut thedeck: Deck = Deck::new_default(); 

    thedeck.shuffle();
    thedeck.print();
}

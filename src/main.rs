use crate::deck::{
    Card,
    CardValues,
    Suits,
    Deck,
};
use rand::Rng;

mod deck;
fn main() {
    let king: Card = Card { value: CardValues::Ace, suit: Suits::Spade }; 
    println!("{}",king); 
    let thedeck: Deck = Deck::new_default(); 

    let mut rng = rand::thread_rng();
    for x in 0..52{
       // let temp: u8= u8::from(rng.gen()) % (52 - x.into());
        let gen: u8 = rng.gen();
        let temp: u8= x + (u8::from(gen) % (52 - x)); 
        println!("{}",temp);
    }
}



use strum::IntoEnumIterator;
use std::fmt::{self, Display};
use rand::Rng;
use crate::enums::{
     Suits,
     CardValues,
};


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
    pub len: u8,
}

impl Deck {
   pub fn new_default() -> Deck{
        //creates 52 card deck in standard order
        let mut cards = Vec::new();
        for new_suit in Suits::iter() {
            for new_value in CardValues::iter() {
                let card = Card { value: new_value, suit: new_suit };
                cards.push(card);
            }
        }
        let len: u8= cards.len() as u8;       

        Deck {
            cards,
            len,
        }
    }
   
   pub fn shuffle(&mut self){
        /*
         * fisher-yates shuffle algo (durstenfeld version)
         */
        
        let mut rng = rand::thread_rng();
        for x in 0..self.len{
            let gen: u8 = rng.gen();    //random u8 integer
            let index: u8= u8::from(gen) % (self.len - x); 
            // println!("x:{}, {}",x,index);
            self.cards.swap(index.into(), (self.len - x - 1).into());
        }

   }
   pub fn draw_card(&mut self) -> Option<Card> {
        self.cards.pop()
   }
   pub fn print(&self){
       //prints all cards in deck
        for line in self.cards.iter(){
            println!("{}",line);
        }
   }
}





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

        Deck {
            cards,
        }
    }
   
   pub fn shuffle(&mut self){
        /*
         * Fisher-Yates shuffle algo (Durstenfeld version)
         */
        
        let mut rng = rand::thread_rng();

        for x in 0..52{
            let gen: u8 = rng.gen();
            let outeri = 52 - x - 1; //outer index
            let temp: u8= u8::from(gen) % (outeri + 1); 
            // println!("x:{}, {}",x,temp);
            self.cards.swap(temp.into(), outeri.into());
        }

   }
   pub fn print(&self){
       //prints all 52 cards
        for line in self.cards.iter(){
            println!("{}",line);
        }
   }
}



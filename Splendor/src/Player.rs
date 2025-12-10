use crate::Card::Card;
use crate::Card::Colors;
use std::collections::HashMap;
use std::ops::Deref;
use crate::Piles::{CardPile, StonePile};
use std::rc::Rc;
use std::cell::RefCell;
/*
pub struct Player {
    cards: Vec<Card>,
    stones: HashMap<Colors, i32>,
    nobles: Vec<HashMap<Colors, i32>>,
    reserved_card: Vec<Card>,
    gold_count: i32,
    stone_pile: Rc<RefCell<StonePile>>,
    card_piles_one: Rc<RefCell<CardPile>>,
    card_piles_two: Rc<RefCell<CardPile>>,
    card_piles_three: Rc<RefCell<CardPile>>,
}

impl Player {
    pub fn new(stone_pile: Rc<RefCell<StonePile>>,
               card_piles_one: Rc<RefCell<CardPile>>,
               card_piles_two: Rc<RefCell<CardPile>>,
               card_piles_three: Rc<RefCell<CardPile>>,) -> Self {
        Self {
            cards:Vec::new(),
            stones:HashMap::from([
                (Colors::Red, 0),
                (Colors::Green, 0),
                (Colors::Blue, 0),
                (Colors::Brown, 0),
                (Colors::White, 0)
            ]),
            nobles:Vec::new(),
            reserved_card:Vec::new(),
            gold_count: 0,
            stone_pile,
            card_piles_one,
            card_piles_two,
            card_piles_three
        }

    }


    pub fn count_score(&self) -> i32 {
        let mut total = 0;
        for card in self.cards.iter() {
            total += card.get_score();
        }
        total += 3*self.nobles.len() as i32;
        total
    }

    pub fn count_card_colors(&self) -> HashMap<Colors, i32> {
        let mut all_colors: HashMap<Colors, i32> = HashMap::from([
            (Colors::Red, 0),
            (Colors::Green, 0),
            (Colors::Blue, 0),
            (Colors::Brown, 0),
            (Colors::White, 0)
        ]);
        for card in self.cards.iter() {
            *(all_colors.get_mut(&card.get_color()).unwrap()) += 1;
        }
        all_colors
    }
    pub fn buy_card(&mut self, card_index: usize, pile_index: usize) -> bool {
        let card: &Card = match pile_index {
            1 => &self.card_piles_one.borrow().get_card(card_index).clone(),
            2 => &self.card_piles_two.borrow().get_card(card_index).clone(),
            3 => &self.card_piles_three.borrow().get_card(card_index).clone(),
            _ => return false
        };
        let mut stone_pile = self.stone_pile.borrow_mut();
        let mut requirement: HashMap<Colors, i32> = HashMap::new();
        for (color, count) in card.get_cost() {
            let player_has = self.count_card_colors().get(color).copied().unwrap_or(0);
            requirement.insert(*color, count - player_has);
        }
        let mut diff:i32 = 0;
        for (color, count) in requirement.iter() {
            if count > self.stones.get(color).unwrap() {
                diff += count - self.stones.get(color).unwrap();
            }
        }
        if diff > 0 {
            if self.gold_count >= diff {
                self.gold_count -= diff;
            } else {
                return false;
            }
        }
        for (color, count) in requirement.iter() {
            if count > self.stones.get(color).unwrap() {
                stone_pile.return_stone(*color, *(self.stones.get_mut(color).unwrap()));
                *(self.stones.get_mut(color).unwrap()) = 0;
            } else {
                stone_pile.return_stone(*color, *count);
                *(self.stones.get_mut(color).unwrap()) -= count;
            }

        }
        self.cards.push(card.clone());
        true

    }

    pub fn reserve(&mut self, card_index: usize, pile_index: usize) -> bool {
        if self.reserved_card.len() >= 3 {
            return false;
        }

        let card = match pile_index {
            1 => match self.card_piles_one.borrow_mut().remove(card_index) {
                Some(card) => card,
                None => return false,
            },
            2 => match self.card_piles_two.borrow_mut().remove(card_index) {
                Some(card) => card,
                None => return false,
            },
            3 => match self.card_piles_three.borrow_mut().remove(card_index) {
                Some(card) => card,
                None => return false,
            },
            _ => return false,
        };
        let mut stone_pile = self.stone_pile.borrow_mut();
        if stone_pile.take_gold() {
            self.gold_count += 1;
        }
        self.reserved_card.push(card);
        
        true
    }
    
    pub fn add_noble(&mut self, noble: HashMap<Colors, i32>) {
        self.nobles.push(noble);
    }
    
    
    
}
 */

pub struct Player {
    cards: Vec<Card>,
    pub(crate) stones: HashMap<Colors, i32>,
    nobles: Vec<HashMap<Colors, i32>>,
    pub reserved_card: Vec<Card>,
    gold_count: i32
}

impl Player {
    pub fn new() -> Self {
        Self {
            cards:Vec::new(),
            stones:HashMap::from([
                (Colors::Red, 0),
                (Colors::Green, 0),
                (Colors::Blue, 0),
                (Colors::Brown, 0),
                (Colors::White, 0)
            ]),
            nobles:Vec::new(),
            reserved_card:Vec::new(),
            gold_count: 0,
        }

    }


    pub fn count_score(&self) -> i32 {
        let mut total = 0;
        for card in self.cards.iter() {
            total += card.get_score();
        }
        total += 3*self.nobles.len() as i32;
        total
    }

    pub fn count_card_colors(&self) -> HashMap<Colors, i32> {
        let mut all_colors: HashMap<Colors, i32> = HashMap::from([
            (Colors::Red, 0),
            (Colors::Green, 0),
            (Colors::Blue, 0),
            (Colors::Brown, 0),
            (Colors::White, 0)
        ]);
        for card in self.cards.iter() {
            *(all_colors.get_mut(&card.get_color()).unwrap()) += 1;
        }
        all_colors
    }

    pub fn add_noble(&mut self, noble: HashMap<Colors, i32>) {
        self.nobles.push(noble);
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn add_stone(&mut self, color: Colors) {
        *(self.stones.get_mut(&color).unwrap()) += 1;
    }
    
    pub fn remove_stone(&mut self, color: Colors, num: i32) {
        (*self.stones.get_mut(&color).unwrap()) -= num;
    }
    
    pub fn get_stone(&self) -> HashMap<Colors, i32> {
        self.stones.clone()
    }

    pub fn add_gold(&mut self) {
        self.gold_count += 1;
    }
    pub fn get_gold_count(&self) -> i32 {
        self.gold_count
    }
    
    pub fn remove_gold(&mut self, num: i32) {
        self.gold_count -= num;
    }



    pub fn output(&self) {
        println!("Total Score: {}", self.count_score());
        print!("Stones processed:   ");
        for color in self.stones.keys() {
            if *(self.stones.get(color).unwrap()) != 0 {
                print!("{}: {}  ", color, self.stones.get(color).unwrap());
            }
        }
        print!("\n");
        print!("Color Collected:   ");
        let all_color = self.count_card_colors();
        for color in all_color.keys() {
            if *(all_color.get(color).unwrap()) != 0 {
                print!("{}: {}  ", color, all_color.get(color).unwrap());
            }
        }
        print!("\n");
        println!("Gold Count: {}", self.gold_count);
        print!("\n");
        println!("Reserved Cards:");
        if self.reserved_card.len() == 0 {
            println!("No reserved card yet");
        } else {
            let mut index = 0;
            for card in self.reserved_card.iter() {
                println!("Card {}:", index);
                card.output();
                index += 1;
            }
        }
        println!("---------------------------------------------------------");
    }



}

use crate::Card::Colors;
use crate::Card::Card;
use std::collections::HashMap;
use std::rc::Rc;
use rand::Rng;
use crate::Player::Player;

pub struct StonePile {
    stone_counts: HashMap<Colors, i32>,
    gold_count: i32
}

pub struct CardPile {
    cards_shown: Vec<Card>,
    cards_hidden: Vec<Card>
}

impl StonePile {
    pub fn new(player_num: i32) -> StonePile {
        todo!()
    }

    pub fn take_stone(&mut self, color: Colors) -> bool {
        if *(self.stone_counts.get(&color).unwrap()) <= 0 {
            return false;
        }
        *(self.stone_counts.get_mut(&color).unwrap()) -= 1;
        true
    }

    pub fn return_stone(&mut self, color: Colors, num: i32) {
        *(self.stone_counts.get_mut(&color).unwrap()) += num;
    }

    pub fn take_gold(&mut self) -> bool{
        if (self.gold_count == 0) {
            return false;
        }
        self.gold_count -= 1;
        true
    }

    pub fn return_gold(&mut self) {
        self.gold_count += 1;
    }
}

impl CardPile {
    pub fn new(level:i32) -> CardPile {
        todo!()
    }

    pub fn get_card(&self,index: usize) -> &Card {
        self.cards_shown.get(index).unwrap()
    }
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();

        for i in (1..self.cards_hidden.len()).rev() {
            let j = rng.gen_range(0..=i);
            self.cards_hidden.swap(i, j);
        }
    }

    pub fn setup(&mut self) {
        for i in 0..4 {
            self.cards_shown.push(self.cards_hidden.pop().unwrap());
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<Card> {
        if index >= self.cards_shown.len() {
            return None;
        }

        let removed_card = self.cards_shown.remove(index);

        if let Some(new_card) = self.cards_hidden.pop() {
            self.cards_shown.push(new_card);
        }

        Some(removed_card)
    }

    pub fn output(&self) {
        todo!()
    }


}

use crate::Card::Colors;
use crate::Card::Card;
use std::collections::HashMap;
use std::rc::Rc;
use rand::Rng;
use crate::Player::Player;
use std::collections::HashSet;

pub struct Piles {
    pub stone_pile: StonePile,
    pub card_pile_one: CardPile,
    pub card_pile_two: CardPile,
    pub card_pile_three: CardPile,
    pub noble_pile: NoblePile,
    pub player_num: i32
}


impl Piles {
    pub fn new(player_num: i32) -> Piles {
        Piles {
            stone_pile: StonePile::new(player_num),
            card_pile_one: CardPile::new(1),
            card_pile_two: CardPile::new(2),
            card_pile_three: CardPile::new(3),
            noble_pile: NoblePile::new(),
            player_num

        }
    }

    pub fn setup(&mut self) {
        self.card_pile_one.shuffle();
        self.card_pile_two.shuffle();
        self.card_pile_three.shuffle();
        self.noble_pile.shuffle();
        self.card_pile_one.setup();
        self.card_pile_two.setup();
        self.card_pile_three.setup();
        self.noble_pile.setup(self.player_num);

    }

    pub fn buy_card(&mut self, card_index: usize, pile_index: usize, player: &mut Player) -> bool {
        let card_pile = match pile_index {
            1 => &self.card_pile_one,
            2 => &self.card_pile_two,
            3 => &self.card_pile_three,
            _ => {
                println!("invalid card_pile index, please try again");
                return false;
            }
        };
        if card_index < 0 || card_index > card_pile.shown_size() {
            println!("invalid card index, please try again");
            return false;
        }
        let card: &Card = card_pile.get_card(card_index);
        let mut requirement: HashMap<Colors, i32> = HashMap::new();
        for (color, count) in card.get_cost() {
            let player_has = player.count_card_colors().get(color).copied().unwrap_or(0);
            requirement.insert(*color, count - player_has);
        }
        let mut diff:i32 = 0;
        for (color, count) in requirement.iter() {
            if count > player.get_stone().get(color).unwrap() {
                diff += count - player.get_stone().get(color).unwrap();
            }
        }
        if diff > 0 {
            if player.get_gold_count() >= diff {
                player.remove_gold(diff);
                self.stone_pile.return_gold(diff);
            } else {
                return false;
            }
        }
        for (color, count) in requirement.iter() {
            if count > player.get_stone().get(color).unwrap() {
                self.stone_pile.return_stone(*color, *(player.get_stone().get_mut(color).unwrap()));
                player.remove_stone(*color, *(player.get_stone().get(color).unwrap()));
            } else {
                self.stone_pile.return_stone(*color, *count);
                player.remove_stone(*color, *count);
            }

        }
        player.add_card(card.clone());
        match pile_index {
            1 => &self.card_pile_one.remove(card_index),
            2 => &self.card_pile_two.remove(card_index),
            3 => &self.card_pile_three.remove(card_index),
            _ => return false
        };
        true

    }

    pub fn get_noble(&mut self, player: &mut Player) {
        NoblePile::get_noble(&mut self.noble_pile, player);
    }

    pub fn take_two_stone(&mut self, player: &mut Player, color: Colors) -> bool {
        if (self.stone_pile.get_stone_count(color) <= 1) {
            return false;
        }
        self.stone_pile.take_stone(color);
        self.stone_pile.take_stone(color);
        player.add_stone(color);
        player.add_stone(color);
        true
    }

    pub fn take_three_stone(&mut self, player: &mut Player, colors: HashSet<Colors>) -> bool {
        for color in colors.clone() {
            if (self.stone_pile.get_stone_count(color) <= 0) {
                return false;
            }
        }
        for color in colors.clone() {
            self.stone_pile.take_stone(color);
            player.add_stone(color);
        }
        true
    }

    pub fn reserve(&mut self, card_index: usize, pile_index: usize, player: &mut Player) -> bool {
        if card_index >= 3 {
            println!("invalid card index, please try again");
            return false;
        }
        let card = match pile_index {
            1 => self.card_pile_one.remove(card_index).unwrap(),
            2 => self.card_pile_two.remove(card_index).unwrap(),
            3 => self.card_pile_three.remove(card_index).unwrap(),
            _ => {
                println!("invalid card_pile index, please try again");
                return false;
            },
        };
        if self.stone_pile.take_gold() {
            player.add_gold();
        }
        player.reserved_card.push(card);
        true
    }


    pub fn output(&self) {
        todo!()
    }

}

pub struct StonePile {
    stone_counts: HashMap<Colors, i32>,
    gold_count: i32
}

#[derive(Clone)]
pub struct CardPile {
    cards_shown: Vec<Card>,
    cards_hidden: Vec<Card>
}

impl StonePile {
    pub fn new(player_num: i32) -> StonePile {
        let stone_num = match player_num {
            2 => 4,
            3 => 5,
            4 => 7,
            _ => 0
        };
        StonePile {
            stone_counts: HashMap::from([
                (Colors::Red, stone_num),
                (Colors::Green, stone_num),
                (Colors::Blue, stone_num),
                (Colors::Brown, stone_num),
                (Colors::White, stone_num)
            ]),
            gold_count: 5
        }
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

    pub fn return_gold(&mut self, num: i32) {
        self.gold_count += num;
    }

    pub fn get_stone_count(&self, color: Colors) -> i32 {
        *self.stone_counts.get(&color).unwrap_or(&0)
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
    
    pub fn shown_size(&self) -> usize { self.cards_shown.len()  }

    pub fn output(&self) {
        todo!()
    }


}

struct NoblePile {
    nobles_shown: Vec<HashMap<Colors, i32>>,
    nobles_hidden: Vec<HashMap<Colors, i32>>
}

impl NoblePile {
    pub fn new() -> NoblePile {
        todo!()
    }
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();

        for i in (1..self.nobles_hidden.len()).rev() {
            let j = rng.gen_range(0..=i);
            self.nobles_hidden.swap(i, j);
        }
    }

    pub fn setup(&mut self, player_num: i32) {
        for i in 0..(player_num+1) {
            self.nobles_shown.push(self.nobles_hidden.pop().unwrap());
        }
    }

    pub fn get_noble(&mut self, player: &mut Player) {
        let player_colors = player.count_card_colors();
        let mut fulfilled: bool = true;
        let mut index: usize = 0;
        let mut index_list: Vec<usize> = Vec::new();
        for noble in self.nobles_shown.iter() {
            for color in noble.keys() {
                if noble.get(color) > player_colors.get(color) {
                    fulfilled = false;
                }
            }
            if fulfilled {
                index_list.push(index);
            }
            index += 1;
        }
        index_list.reverse();
        for index in index_list {
            player.add_noble(self.nobles_shown.remove(index));
        }

    }

}

pub struct PlayerPile {
    players: Vec<Player>,
    next: usize
}

impl PlayerPile {
    pub fn new(player_num: i32) -> PlayerPile {
        let mut vec: Vec<Player> = Vec::new();
        for i in 0..player_num {
            vec.push(Player::new());
        }
        PlayerPile {
            players: vec,
            next: 0
        }
    }
}



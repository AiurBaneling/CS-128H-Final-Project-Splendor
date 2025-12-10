use crate::Card::Colors;
use crate::Card::Card;
use std::collections::HashMap;
use std::rc::Rc;
use rand::Rng;
use crate::Player::Player;
use std::collections::HashSet;
use std::{io, thread};
use std::time::Duration;

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
                thread::sleep(Duration::from_secs(1));
                return false;
            }
        };
        if card_index < 0 || card_index > card_pile.shown_size() {
            println!("invalid card index, please try again");
            thread::sleep(Duration::from_secs(1));
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
                println!("You don't have enough stone to buy the card");
                thread::sleep(Duration::from_secs(1));
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

    pub fn buy_reserved_card(&mut self, card_index: usize, player: &mut Player) -> bool {
        if card_index >= player.reserved_card.len() {
            println!("Invalid card index, please try again");
            thread::sleep(Duration::from_secs(1));
            return false;
        }
        let card = &player.reserved_card[card_index].clone();
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
                println!("You don't have enough stone to buy the card");
                thread::sleep(Duration::from_secs(1));
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
        player.reserved_card.remove(card_index);
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
            thread::sleep(Duration::from_secs(1));
            return false;
        }
        let card = match pile_index {
            1 => self.card_pile_one.remove(card_index).unwrap(),
            2 => self.card_pile_two.remove(card_index).unwrap(),
            3 => self.card_pile_three.remove(card_index).unwrap(),
            _ => {
                println!("invalid card_pile index, please try again");
                thread::sleep(Duration::from_secs(1));
                return false;
            },
        };
        if self.stone_pile.take_gold() {
            player.add_gold();
        }
        player.reserved_card.push(card);
        true
    }

    pub fn check_capacity(&mut self, player: &mut Player) {
        if player.stones.len() > 10 {
            let mut diff = player.stones.len() - 10;
            println!("You have too many stones!");
            println!("You need to remove {} stones", diff);
            while diff != 0 {
                println!("Please select a color to remove: 1.red 2.green 3.blue 4.brown 5.white");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read");
                let num: i32 = input.trim().parse().expect("Please enter a number");
                if (num > 5 || num < 1 ) {
                    println!("invalid color, please try again");
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }
                let color = match num {
                    1 => Colors::Red,
                    2 => Colors::Green,
                    3 => Colors::Blue,
                    4 => Colors::Brown,
                    5 => Colors::White,
                    _ => unreachable!("pile_index should be 1-5")
                };
                if *(player.stones.get(&color).unwrap()) == 0 {
                    println!("You don't have such stone");
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }
                *(player.stones.get_mut(&color).unwrap()) -= 1;
                self.stone_pile.return_stone(color, 1);
                diff -= 1;
            }
        }
    }



    pub fn output(&self) {
        println!("---------------------------------------------------------");
        println!("The Nobles:");
        self.noble_pile.output();
        println!("---------------------------------------------------------");
        println!("Level 3 Pile:");
        self.card_pile_three.output();
        println!("---------------------------------------------------------");
        println!("Level 2 Pile:");
        self.card_pile_two.output();
        println!("---------------------------------------------------------");
        println!("Level 1 Pile:");
        self.card_pile_one.output();
        println!("---------------------------------------------------------");
        println!("Stone Pile: ");
        self.stone_pile.output();
        println!("---------------------------------------------------------");
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

    pub fn output(&self) {
        println!("Red: {}, Green: {}, Blue: {}, Brown: {}, White: {}", self.stone_counts.get(&Colors::Red).unwrap(), self.stone_counts.get(&Colors::Green).unwrap(), self.stone_counts.get(&Colors::Blue).unwrap(),self.stone_counts.get(&Colors::Brown).unwrap(),self.stone_counts.get(&Colors::White).unwrap());
        println!("Gold: {}", self.gold_count);
    }
}

impl CardPile {
    pub fn new(level:i32) -> CardPile {
        match level {
            3 => return CardPile {
                cards_shown: Vec::new(),
                cards_hidden: Vec::from([
                    Card::new(Colors::Green, 3, HashMap::from([
                        (Colors::Red, 3),
                        (Colors::Green, 0),
                        (Colors::Blue, 3),
                        (Colors::Brown, 3),
                        (Colors::White, 5)
                    ])),
                    Card::new(Colors::Red, 3, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 3),
                        (Colors::Blue, 5),
                        (Colors::Brown, 3),
                        (Colors::White, 3)
                    ])),
                    Card::new(Colors::Blue, 3, HashMap::from([
                        (Colors::Red, 3),
                        (Colors::Green, 3),
                        (Colors::Blue, 0),
                        (Colors::Brown, 5),
                        (Colors::White, 3)
                    ])),
                    Card::new(Colors::White, 5, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 7),
                        (Colors::White, 3)
                    ])),
                    Card::new(Colors::White, 4, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 7),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Green, 5, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 3),
                        (Colors::Blue, 7),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Blue, 4, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 3),
                        (Colors::Brown, 3),
                        (Colors::White, 6)
                    ])),
                    Card::new(Colors::Red, 4, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 7),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::White, 3, HashMap::from([
                        (Colors::Red, 5),
                        (Colors::Green, 3),
                        (Colors::Blue, 3),
                        (Colors::Brown, 3),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Green, 4, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 7),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Brown, 4, HashMap::from([
                        (Colors::Red, 6),
                        (Colors::Green, 3),
                        (Colors::Blue, 0),
                        (Colors::Brown, 3),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Green, 4, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 3),
                        (Colors::Blue, 6),
                        (Colors::Brown, 0),
                        (Colors::White, 3)
                    ])),
                    Card::new(Colors::Brown, 3, HashMap::from([
                        (Colors::Red, 3),
                        (Colors::Green, 5),
                        (Colors::Blue, 3),
                        (Colors::Brown, 0),
                        (Colors::White, 5)
                    ])),
                    Card::new(Colors::Red, 4, HashMap::from([
                        (Colors::Red, 3),
                        (Colors::Green, 6),
                        (Colors::Blue, 3),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Brown, 4, HashMap::from([
                        (Colors::Red, 7),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Blue, 4, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 7)
                    ])),
                    Card::new(Colors::Blue, 5, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 3),
                        (Colors::Brown, 0),
                        (Colors::White, 7)
                    ])),
                    Card::new(Colors::Brown, 5, HashMap::from([
                        (Colors::Red, 7),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 3),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::White, 4, HashMap::from([
                        (Colors::Red, 3),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 6),
                        (Colors::White, 3)
                    ])),
                    Card::new(Colors::Red, 5, HashMap::from([
                        (Colors::Red, 3),
                        (Colors::Green, 7),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                ]),
            },
            2 => return CardPile {
                cards_shown: Vec::new(),
                cards_hidden: Vec::from([
                    Card::new(Colors::Blue, 3, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 6),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Red, 2, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 5),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Red, 2, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 2),
                        (Colors::Blue, 4),
                        (Colors::Brown, 0),
                        (Colors::White, 1)
                    ])),
                    Card::new(Colors::White, 3, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 6)
                    ])),
                    Card::new(Colors::Blue, 1, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 3),
                        (Colors::Blue, 2),
                        (Colors::Brown, 3),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Brown, 3, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 6),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Blue, 2, HashMap::from([
                        (Colors::Red, 1),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 4),
                        (Colors::White, 2)
                    ])),
                    Card::new(Colors::Blue, 2, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 3),
                        (Colors::Brown, 0),
                        (Colors::White, 5)
                    ])),
                    Card::new(Colors::White, 2, HashMap::from([
                        (Colors::Red, 4),
                        (Colors::Green, 1),
                        (Colors::Blue, 0),
                        (Colors::Brown, 2),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::White, 2, HashMap::from([
                        (Colors::Red, 5),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Green, 2, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 2),
                        (Colors::Brown, 1),
                        (Colors::White, 4)
                    ])),
                    Card::new(Colors::Green, 3, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 6),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Blue, 2, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 5),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Green, 2, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 3),
                        (Colors::Blue, 5),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Green, 2, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 5),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Blue, 1, HashMap::from([
                        (Colors::Red, 3),
                        (Colors::Green, 2),
                        (Colors::Blue, 3),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::White, 1, HashMap::from([
                        (Colors::Red, 2),
                        (Colors::Green, 3),
                        (Colors::Blue, 0),
                        (Colors::Brown, 2),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Brown, 2, HashMap::from([
                        (Colors::Red, 2),
                        (Colors::Green, 4),
                        (Colors::Blue, 1),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Brown, 1, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 3),
                        (Colors::Blue, 0),
                        (Colors::Brown, 2),
                        (Colors::White, 3)
                    ])),
                    Card::new(Colors::Brown, 1, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 2),
                        (Colors::Blue, 2),
                        (Colors::Brown, 0),
                        (Colors::White, 3)
                    ])),
                    Card::new(Colors::Brown, 2, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 5)
                    ])),
                    Card::new(Colors::Green, 1, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 3),
                        (Colors::Brown, 2),
                        (Colors::White, 2)
                    ])),
                    Card::new(Colors::Red,3, HashMap::from([
                        (Colors::Red, 6),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Brown, 2, HashMap::from([
                        (Colors::Red, 3),
                        (Colors::Green, 5),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::White, 1, HashMap::from([
                        (Colors::Red, 3),
                        (Colors::Green, 0),
                        (Colors::Blue, 3),
                        (Colors::Brown, 0),
                        (Colors::White, 2)
                    ])),
                    Card::new(Colors::Red, 1, HashMap::from([
                        (Colors::Red, 2),
                        (Colors::Green, 0),
                        (Colors::Blue, 3),
                        (Colors::Brown, 3),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Red, 1, HashMap::from([
                        (Colors::Red, 2),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 3),
                        (Colors::White, 2)
                    ])),
                    Card::new(Colors::Green,1, HashMap::from([
                        (Colors::Red, 3),
                        (Colors::Green, 2),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 3)
                    ])),
                    Card::new(Colors::White, 2, HashMap::from([
                        (Colors::Red, 5),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 3),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Red, 2, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 5),
                        (Colors::White, 3)
                    ])),
                ]),
            },
            1 => return CardPile {
                cards_shown: Vec::new(),
                cards_hidden: Vec::from([
                    Card::new(Colors::Blue, 0, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 3),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Blue, 1, HashMap::from([
                        (Colors::Red, 4),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::White, 0, HashMap::from([
                        (Colors::Red, 1),
                        (Colors::Green, 1),
                        (Colors::Blue, 1),
                        (Colors::Brown, 1),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::White, 0, HashMap::from([
                        (Colors::Red, 2),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 1),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::White, 0, HashMap::from([
                        (Colors::Red, 1),
                        (Colors::Green, 2),
                        (Colors::Blue, 1),
                        (Colors::Brown, 1),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Green,0, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 1),
                        (Colors::Blue, 3),
                        (Colors::Brown, 0),
                        (Colors::White, 1)
                    ])),
                    Card::new(Colors::Green,0, HashMap::from([
                        (Colors::Red, 2),
                        (Colors::Green, 0),
                        (Colors::Blue, 1),
                        (Colors::Brown, 2),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Green,1, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 4),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Blue, 0, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 2),
                        (Colors::White, 1)
                    ])),
                    Card::new(Colors::Blue, 0, HashMap::from([
                        (Colors::Red, 2),
                        (Colors::Green, 1),
                        (Colors::Blue, 0),
                        (Colors::Brown, 1),
                        (Colors::White, 1)
                    ])),
                    Card::new(Colors::Blue, 0, HashMap::from([
                        (Colors::Red, 1),
                        (Colors::Green, 3),
                        (Colors::Blue, 1),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Red, 0, HashMap::from([
                        (Colors::Red, 1),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 3),
                        (Colors::White, 1)
                    ])),
                    Card::new(Colors::Red, 1, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 4)
                    ])),
                    Card::new(Colors::Red,0, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 1),
                        (Colors::Blue, 1),
                        (Colors::Brown, 1),
                        (Colors::White, 2)
                    ])),
                    Card::new(Colors::Red, 0, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 1),
                        (Colors::Blue, 1),
                        (Colors::Brown, 1),
                        (Colors::White, 1)
                    ])),
                    Card::new(Colors::White, 0, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 1),
                        (Colors::Brown, 1),
                        (Colors::White, 3)
                    ])),
                    Card::new(Colors::White, 0, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 3),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Green, 0, HashMap::from([
                        (Colors::Red, 2),
                        (Colors::Green, 0),
                        (Colors::Blue, 2),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Green, 0, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 1),
                        (Colors::Brown, 0),
                        (Colors::White, 2)
                    ])),
                    Card::new(Colors::Brown, 0, HashMap::from([
                        (Colors::Red, 1),
                        (Colors::Green, 0),
                        (Colors::Blue, 2),
                        (Colors::Brown, 0),
                        (Colors::White, 2)
                    ])),
                    Card::new(Colors::Brown, 0, HashMap::from([
                        (Colors::Red, 1),
                        (Colors::Green, 1),
                        (Colors::Blue, 1),
                        (Colors::Brown, 0),
                        (Colors::White, 1)
                    ])),
                    Card::new(Colors::Brown, 0, HashMap::from([
                        (Colors::Red, 3),
                        (Colors::Green, 1),
                        (Colors::Blue, 0),
                        (Colors::Brown, 1),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Brown, 0, HashMap::from([
                        (Colors::Red, 1),
                        (Colors::Green, 2),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Blue, 0, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 2),
                        (Colors::Blue, 0),
                        (Colors::Brown, 2),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::White, 0, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 2),
                        (Colors::Brown, 2),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Brown, 0, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 2),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 2)
                    ])),
                    Card::new(Colors::Brown, 1, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 4),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Brown, 0, HashMap::from([
                        (Colors::Red, 1),
                        (Colors::Green, 1),
                        (Colors::Blue, 2),
                        (Colors::Brown, 0),
                        (Colors::White, 1)
                    ])),
                    Card::new(Colors::Green, 0, HashMap::from([
                        (Colors::Red, 1),
                        (Colors::Green, 0),
                        (Colors::Blue, 1),
                        (Colors::Brown, 1),
                        (Colors::White, 1)
                    ])),
                    Card::new(Colors::White, 0, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 2),
                        (Colors::Blue, 2),
                        (Colors::Brown, 1),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::White, 1, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 4),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Red, 0, HashMap::from([
                        (Colors::Red, 2),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 2)
                    ])),
                    Card::new(Colors::Green, 0, HashMap::from([
                        (Colors::Red, 3),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Green, 0, HashMap::from([
                        (Colors::Red, 1),
                        (Colors::Green, 0),
                        (Colors::Blue, 1),
                        (Colors::Brown, 2),
                        (Colors::White, 1)
                    ])),
                    Card::new(Colors::Red, 0, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 0),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 3)
                    ])),
                    Card::new(Colors::Red, 0, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 1),
                        (Colors::Blue, 2),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                    Card::new(Colors::Blue, 0, HashMap::from([
                        (Colors::Red, 2),
                        (Colors::Green, 2),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 1)
                    ])),
                    Card::new(Colors::Blue, 0, HashMap::from([
                        (Colors::Red, 1),
                        (Colors::Green, 1),
                        (Colors::Blue, 0),
                        (Colors::Brown, 1),
                        (Colors::White, 1)
                    ])),
                    Card::new(Colors::Red, 0, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 1),
                        (Colors::Blue, 0),
                        (Colors::Brown, 2),
                        (Colors::White, 2)
                    ])),
                    Card::new(Colors::Brown, 0, HashMap::from([
                        (Colors::Red, 0),
                        (Colors::Green, 3),
                        (Colors::Blue, 0),
                        (Colors::Brown, 0),
                        (Colors::White, 0)
                    ])),
                ]),
            },
            _ => unreachable!(),
        }
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
        println!("Card 0:");
        self.cards_shown.get(0).unwrap().output();
        println!("Card 1:");
        self.cards_shown.get(1).unwrap().output();
        println!("Card 2:");
        self.cards_shown.get(2).unwrap().output();
        println!("Card 3:");
        self.cards_shown.get(3).unwrap().output();
    }


}

struct NoblePile {
    nobles_shown: Vec<HashMap<Colors, i32>>,
    nobles_hidden: Vec<HashMap<Colors, i32>>
}

impl NoblePile {
    pub fn new() -> NoblePile {
        NoblePile {
            nobles_shown: Vec::new(),
            nobles_hidden: Vec::from([
                HashMap::from([(Colors::Red, 3),
                    (Colors::Green, 3),
                    (Colors::Blue, 0),
                    (Colors::Brown, 3),
                    (Colors::White, 0)]),
                HashMap::from([(Colors::Red, 0),
                    (Colors::Green, 0),
                    (Colors::Blue, 3),
                    (Colors::Brown, 3),
                    (Colors::White, 3)]),
                HashMap::from([(Colors::Red, 3),
                    (Colors::Green, 0),
                    (Colors::Blue, 0),
                    (Colors::Brown, 3),
                    (Colors::White, 3)]),
                HashMap::from([(Colors::Red, 0),
                    (Colors::Green, 3),
                    (Colors::Blue, 3),
                    (Colors::Brown, 0),
                    (Colors::White, 3)]),
                HashMap::from([(Colors::Red, 3),
                    (Colors::Green, 3),
                    (Colors::Blue, 3),
                    (Colors::Brown, 0),
                    (Colors::White, 0)]),
                HashMap::from([(Colors::Red, 4),
                    (Colors::Green, 0),
                    (Colors::Blue, 0),
                    (Colors::Brown, 4),
                    (Colors::White, 0)]),
                HashMap::from([(Colors::Red, 4),
                    (Colors::Green, 4),
                    (Colors::Blue, 0),
                    (Colors::Brown, 0),
                    (Colors::White, 0)]),
                HashMap::from([(Colors::Red, 0),
                    (Colors::Green, 4),
                    (Colors::Blue, 4),
                    (Colors::Brown, 0),
                    (Colors::White, 0)]),
                HashMap::from([(Colors::Red, 0),
                    (Colors::Green, 0),
                    (Colors::Blue, 0),
                    (Colors::Brown, 4),
                    (Colors::White, 4)]),
                HashMap::from([(Colors::Red, 0),
                    (Colors::Green, 0),
                    (Colors::Blue, 4),
                    (Colors::Brown, 0),
                    (Colors::White, 4)])])
        }

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

    pub fn output(&self) {
        let mut index: usize = 0;
        for noble in self.nobles_shown.iter() {
            print!("Noble {}: ", index);
            for color in noble.keys() {
                if *(noble.get(color).unwrap()) != 0 {
                    print!("{}: {}  ", color, noble.get(color).unwrap());
                }
            }
            print!("\n");
            index += 1;
        }
    }

}


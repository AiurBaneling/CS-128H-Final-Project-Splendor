use crate::Player::Player;
use std::rc::Rc;
use std::cell::RefCell;
use crate::Card::Card;
use crate::Card::Colors;
use std::collections::HashMap;
use std::ops::Deref;
use crate::Noble::Noble;
use crate::Piles::{CardPile, StonePile};

pub fn setup(num: i32) ->Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    let stone_pile = Rc::new(RefCell::new(StonePile::new(num)));
    let card_pile_one = Rc::new(RefCell::new(CardPile::new(1)));
    let card_pile_two = Rc::new(RefCell::new(CardPile::new(2)));
    let card_pile_three = Rc::new(RefCell::new(CardPile::new(3)));
    for i in 0..num {
        players.push(Player::new(Rc::clone(&stone_pile), Rc::clone(&card_pile_one), Rc::clone(&card_pile_two), Rc::clone(&card_pile_three)));
    }
    players
}
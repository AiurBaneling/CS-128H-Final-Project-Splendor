use crate::Player::Player;
use std::rc::Rc;
use std::cell::RefCell;
use crate::Card::Card;
use crate::Card::Colors;
use std::collections::HashMap;
use std::ops::Deref;
use crate::Piles::{CardPile, Piles, StonePile};
use std::io;

pub fn setup() ->(Vec<Player>, Piles) {
    println!("Please enter the number of players (2-4): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    let mut num: i32 = input.trim().parse().expect("Please enter a number");
    while (num > 4 || num < 2) {
        println!("Please enter a valid number");
        println!("Please enter the number of players (2-4): ");
        input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        num = input.trim().parse().expect("Please enter a number");
    }

    let mut players: Vec<Player> = Vec::new();
    for i in 0..num {
        players.push(Player::new());
    }
    
    let pile = Piles::new(num);

    (players, pile)

}
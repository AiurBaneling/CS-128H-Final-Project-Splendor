use std::collections::HashSet;
use std::io;
use crate::Card::Colors;
use std::thread;
use std::time::Duration;

mod Card;
mod Player;
mod Piles;
mod mainlib;

fn main() {

    let mut players: Vec<Player::Player> = Vec::new();
    let mut pile: Piles::Piles;

    (players, pile) = mainlib::setup();
    let mut curr: usize = 0;
    pile.setup();


    loop {
        let mut success = false;
        let mut player = players.get_mut(curr).unwrap();
        pile.output();
        println!("Player {} information:", curr);
        player.output();
        println!("Player {}", curr);
        println!("What action would you like to do? \n1. Take three different stone\n2. Take two identical stone\n3. Buy a card\n4. Reserve a card\n5. Buy a reserved card");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        let num: i32 = input.trim().parse().expect("Please enter a number");
        match num {
            1 => {
                let mut color_set: HashSet<Colors> = HashSet::new();
                let mut i = 0;
                while i<3 {
                    println!("Please enter a color: 1.red 2.green 3.blue 4.brown 5.white");
                    input.clear();
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
                    if color_set.contains(&color) {
                        println!("color already choosed, please try again");
                        thread::sleep(Duration::from_secs(1));
                        continue;
                    }
                    color_set.insert(color);
                    i += 1;
                }
                if pile.take_three_stone(player, color_set) {
                    println!("successful action");
                    thread::sleep(Duration::from_secs(1));
                    success = true;
                } else {
                    println!("not enough stone in these colors, please try again");
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }
            },
            2 => {
                let mut color:Colors;
                loop {
                    println!("Please enter a color: 1.red 2.green 3.blue 4.brown 5.white");
                    input.clear();
                    io::stdin().read_line(&mut input).expect("Failed to read");
                    let num: i32 = input.trim().parse().expect("Please enter a number");
                    if (num > 5 || num < 1 ) {
                        println!("invalid color, please try again");
                        thread::sleep(Duration::from_secs(1));
                        continue;
                    }
                    color = match num {
                        1 => Colors::Red,
                        2 => Colors::Green,
                        3 => Colors::Blue,
                        4 => Colors::Brown,
                        5 => Colors::White,
                        _ => unreachable!("pile_index should be 1-5")
                    };
                    break;
                }
                if pile.take_two_stone(player, color) {
                    println!("successful action");
                    thread::sleep(Duration::from_secs(1));
                    success = true;
                } else {
                    println!("not enough stone in this color, please try again");
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }

            },
            3 => {
                loop {
                    println!("Please enter which pile you are buying from");
                    input.clear();
                    io::stdin().read_line(&mut input).expect("Failed to read");
                    let mut num: usize = input.trim().parse().expect("Please enter a number");
                    let pile_index = num;
                    println!("Please enter which card you are buying");
                    input.clear();
                    io::stdin().read_line(&mut input).expect("Failed to read");
                    num = input.trim().parse().expect("Please enter a number");
                    let card_index = num;
                    if pile.buy_card(card_index, pile_index, player) {
                        println!("successful action");
                        thread::sleep(Duration::from_secs(1));
                        success = true;
                        break;
                    } else {
                        thread::sleep(Duration::from_secs(1));
                        break;
                    }
                }
            },
            4 => {
                if player.reserved_card.len() >= 3 {
                    println!("too much reserved card, please try other action");
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }
                loop {
                    println!("Please enter which pile you are reserving from");
                    input.clear();
                    io::stdin().read_line(&mut input).expect("Failed to read");
                    let mut num: usize = input.trim().parse().expect("Please enter a number");
                    let pile_index = num;
                    println!("Please enter which card you are reserving");
                    input.clear();
                    io::stdin().read_line(&mut input).expect("Failed to read");
                    num = input.trim().parse().expect("Please enter a number");
                    let card_index = num;
                    if pile.reserve(card_index, pile_index, player) {
                        println!("successful action");
                        thread::sleep(Duration::from_secs(1));
                        success = true;
                        break;
                    } else {
                        break;
                    }
                }
            },
            5 => {
                if player.reserved_card.len() == 0 {
                    println!("You don't have any reserved card yet");
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }
                loop {
                    println!("Please enter which card you are going to buy");
                    input.clear();
                    io::stdin().read_line(&mut input).expect("Failed to read");
                    let num: usize = input.trim().parse().expect("Please enter a number");
                    let card_index = num;
                    if pile.buy_reserved_card(card_index, player) {
                        println!("successful action");
                        thread::sleep(Duration::from_secs(1));
                        success = true;
                        break;
                    } else {
                        break;
                    }
                }

            }
            _ => {
                println!("Please enter a valid number");
                continue;
            }
        }
        pile.get_noble(player);
        if player.count_score() >= 15 {
            println!("We have a winner!");
            println!("The winner is player {}", curr+1);
            break;
        }
        pile.check_capacity(player);
        if success {
            curr = (curr + 1) % players.len();
        }

    }

}

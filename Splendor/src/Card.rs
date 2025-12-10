use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Colors {
    Red,
    Green,
    Blue,
    Brown,
    White,
}

impl fmt::Display for Colors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Colors::Red => write!(f, "Red"),
            Colors::Green => write!(f, "Green"),
            Colors::Blue => write!(f, "Blue"),
            Colors::Brown => write!(f, "Brown"),
            Colors::White => write!(f, "White"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Card {
    pub(crate) color: Colors,
    pub(crate) score: i32,
    pub(crate) cost: HashMap<Colors,i32>,
}

impl Card {
    pub fn new(color: Colors, score: i32, cost: HashMap<Colors, i32>) -> Card {
        Card {
            color,
            score,
            cost,
        }
    }
    pub fn get_color(&self) -> &Colors {
        &(self.color)
    }
    pub fn get_score(&self) -> i32 {
        self.score
    }
    
    pub fn get_cost(&self) -> &HashMap<Colors,i32> {
        &self.cost
    }

    pub fn output(&self) {
        println!("Color: {}, score: {}", self.color, self.score);
        print!("Costs:   ");
        for color in self.cost.keys() {
            if *(self.cost.get(color).unwrap()) != 0 {
                print!("{}: {}  ", color, self.cost.get(color).unwrap());
            }
        }
        print!("\n \n");
    }

}
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Colors {
    Red,
    Green,
    Blue,
    Brown,
    White,
}

#[derive(Debug, Clone)]
pub struct Card {
    color: Colors,
    score: i32,
    cost: HashMap<Colors,i32>,
    level: i32
}

impl Card {
    pub fn new(color: Colors, score: i32, cost: HashMap<Colors, i32>, level: i32) -> Card {
        Card {
            color,
            score,
            cost,
            level
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
    
    pub fn get_level(&self) -> i32 {
        self.level
    }
    
    
    
    
    

    


}
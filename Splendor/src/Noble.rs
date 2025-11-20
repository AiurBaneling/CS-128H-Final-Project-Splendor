use crate::Card::Colors;
use std::collections::HashMap;
pub struct Noble {
    requirement: HashMap<Colors, i32>
}

impl Noble {
    pub fn new(requirement: HashMap<Colors, i32>) -> Noble {
        Noble {
            requirement
        }
    }
    
    
    pub fn get_requirement(&self) -> &HashMap<Colors, i32> {
        &self.requirement
    }
}
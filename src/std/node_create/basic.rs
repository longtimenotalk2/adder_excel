use std::fmt::Debug;

use colorful::{Color, Colorful};

use crate::std::node_create::LogicBlockCreateError;

impl LogicBlockCreateError {
    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        ret += &format!("hint : {:?}\n", self.hint);
        ret += "founded : \n";
        for wire in &self.found_wires {
            ret += &format!("> {}\n", format!("{:?}", wire).color(Color::Green));
        }
        ret += "expected but not match: \n";
        ret += &format!("? {}", format!("{:?}", self.misfound_wire).color(Color::Red));
        ret
    }
}

impl Debug for LogicBlockCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
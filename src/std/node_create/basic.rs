use std::fmt::Debug;

use crate::std::node_create::LogicBlockCreateError;

impl LogicBlockCreateError {
    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        ret += &format!("hint : {:?}\n", self.hint);
        ret += "founded : \n";
        for wire in &self.found_wires {
            ret += &format!("> {:?}\n", wire);
        }
        ret += "expected but not match: \n";
        ret += &format!("{:?}", self.misfound_wire);
        ret
    }
}

impl Debug for LogicBlockCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
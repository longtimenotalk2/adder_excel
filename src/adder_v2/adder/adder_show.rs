use colorful::{Color, Colorful};

use crate::adder_v2::adder::Adder;

impl Adder {
    pub fn to_string(&self) -> String {
        let mut txt = format!("Adder: {} bit :\n", self.bits);
        txt += "wire list :\n"; 
        for wire in &self.wires {
            txt += &format!("> {:03} : {}\n", wire.0, wire.1.to_string());
        }
        txt += "cell list :\n";
        for (id, cell) in self.cells.iter() {
            txt += &format!("> {id:03} : {}\n", cell.to_string());
        }
        txt
    }
}
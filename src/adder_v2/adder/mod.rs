mod adder_create;
mod adder_show;
pub mod adder_check_id;
pub mod adder_function;
pub mod adder_property;
pub mod adder_cap;

use colorful::{Color, Colorful};

use crate::adder_v2::{cell::Cell, logic::Logic, node::Node, wire::Wire, Id};


#[derive(Debug, Clone)]
pub struct Adder {
    pub wires : Vec<(Id, Wire)>,
    pub cells : Vec<(Id, Cell)>,
    pub bits : usize,
    pub input_is_neg : bool,
    pub output_is_neg : bool,
}

impl Adder {
    pub fn polar_name_lowercase(&self) -> String {
        let mut ret = String::new();
        if self.input_is_neg {
            ret.push_str("n");
        }  else {
            ret.push_str("p");
        }
        if self.output_is_neg {
            ret.push_str("n");
        } else {
            ret.push_str("p");
        }
        ret
    }
}
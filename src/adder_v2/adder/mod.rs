mod adder_create;
mod adder_show;
pub mod adder_check_id;
pub mod adder_function;
pub mod adder_property;

use colorful::{Color, Colorful};

use crate::adder_v2::{cell::Cell, logic::Logic, node::Node, wire::Wire, Id};


#[derive(Debug, Clone)]
pub struct Adder {
    wires : Vec<(Id, Wire)>,
    cells : Vec<(Id, Cell)>,
    bits : usize,
    input_is_neg : bool,
    output_is_neg : bool,
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
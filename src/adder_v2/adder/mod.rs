mod adder_create;
mod adder_show;
pub mod adder_check_id;
pub mod adder_function;
pub mod adder_property;

use colorful::{Color, Colorful};

use crate::adder_v2::{cell_info::CellInfo, logic::Logic, node::Node, wire::Wire, Id};

/// 一个接好了线的Cell
#[derive(Debug, Clone)]
pub struct Cell {
    pub node: Node,
    pub info: CellInfo,
}

impl Cell {
    pub fn new(node: Node, info: CellInfo) -> Self {
        Self {
            node,
            info
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", self.node.to_string(), if self.info.to_string().len() > 0 { format!("[{}]", self.info.to_string().color(Color::Yellow)) } else { "".to_string() })
    }

    pub fn to_cell_body(&self) -> CellBody {
        CellBody {
            logic: self.node.logic.clone(),
            info: self.info.clone(),
        }
    }
}

/// 一个没有任何接线的Cell
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CellBody {
    pub logic: Logic,
    pub info: CellInfo,
}

impl CellBody {
    pub fn to_string(&self) -> String {
        format!("{} {}", self.logic.to_string(), if self.info.to_string().len() > 0 { format!("[{}]", self.info.to_string().color(Color::Yellow)) } else { "".to_string() })
    }
}


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
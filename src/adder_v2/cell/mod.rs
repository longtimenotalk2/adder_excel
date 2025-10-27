pub mod cell_info;

use colorful::{Color, Colorful};

use crate::adder_v2::{cell::cell_info::CellInfo, logic::Logic, node::Node};

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
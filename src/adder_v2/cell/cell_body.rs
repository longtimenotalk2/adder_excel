use colorful::{Color, Colorful};

use crate::adder_v2::{cell::cell_info::CellInfo, logic::Logic};

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
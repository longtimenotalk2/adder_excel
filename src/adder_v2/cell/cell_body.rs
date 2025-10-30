use colorful::{Color, Colorful};

use crate::adder_v2::{cell::cell_info::{CellInfo, Drive}, logic::Logic, Port};

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

    pub fn cap_cmos_given_input_port(&self, _port : &Port) -> i32 {
        let n = match self.logic {
            Logic::XNR2 | Logic::XOR2 | Logic::XNR2DOUT | Logic::XOR2DOUT => 4,
            _ => 2,
        };
        match self.info.drive {
            Drive::D1 => n,
            Drive::D2 => n * 2,
            Drive::D3 => n * 3,
            Drive::D4 => n * 4,
        }
    }

    pub fn cap_cmos_o1_inner(&self) -> i32 {
        let n = match self.logic {
            Logic::XNR2 | Logic::XOR2 | Logic::XNR2DOUT | Logic::XOR2DOUT => 2,
            _ => unimplemented!(),
        };
        n
    }
}
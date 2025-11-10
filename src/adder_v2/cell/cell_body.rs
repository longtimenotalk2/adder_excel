use std::collections::BTreeSet;

use colorful::{Color, Colorful};

use crate::adder_v2::{cell::cell_info::{CellInfo, Drive, SpecialInfo}, logic::Logic, Port};

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

    pub fn cap_cmos_given_input_port(&self, port : &Port) -> i32 {
        let mut n = match self.logic {
            Logic::XNR2 | Logic::XOR2 | Logic::XNR2DOUT | Logic::XOR2DOUT => 4,
            _ => 2,
        };
        if self.contains_special("SP-BD") && port == &Port::new("B") {
            n *= 2;
        }

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

    pub fn contains_special(&self, s : &str) -> bool {
        self.info.special_infos.contains(&SpecialInfo::new(s))
    }

    pub fn mos_num(&self) -> i32 {
        let mut basic_num = match self.logic {
            Logic::INV => 2,
            Logic::ND2 | Logic::NR2 => 4,
            Logic::AOI21 | Logic::OAI21 | Logic::AN2 | Logic::OR2 | Logic::IND2 | Logic::INR2 => 6,
            Logic::AOI22 | Logic::OAI22 | Logic::IAOI21 | Logic::IOAI21 | Logic::AO21 | Logic::OA21 | Logic::AOAI211 | Logic::OAOI211 => 8,
            Logic::XNR2 | Logic::XOR2 | Logic::XNR2DOUT | Logic::XOR2DOUT | Logic::AOA211 | Logic::OAO211 => 10,
        };
        if self.contains_special("SP-BD") {
            basic_num += 2;
        };
        match self.info.drive {
            Drive::D1 => basic_num,
            Drive::D2 => basic_num * 2,
            Drive::D3 => basic_num * 3,
            Drive::D4 => basic_num * 4,
        }
    }
}

#[test]
fn test() {
    let mut infos: BTreeSet<SpecialInfo> = BTreeSet::new();

    infos.insert(SpecialInfo::new("VDH"));
    infos.insert(SpecialInfo::new("VTPUL"));
    infos.insert(SpecialInfo::new("SP-BD"));


    let body = CellBody { logic : Logic::OAI21, info : CellInfo { drive : Drive::D1, special_infos: infos } };
    dbg!(body.cap_cmos_given_input_port(&Port::new("B")));
    dbg!(body.cap_cmos_given_input_port(&Port::new("A1")));
}
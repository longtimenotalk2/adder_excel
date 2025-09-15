pub mod high_match_check;
pub mod spice_cell_demand;

use std::collections::BTreeSet;

use crate::std::logic_block::Port;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DualVddType {
    L2H,
    H2H,
    H2L(BTreeSet<Port>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DualVddDemand {
    dual_vdd_type : DualVddType,
}

impl DualVddDemand {
    pub fn from_excel_string(s : &str) -> Self {
        match s {
            "H" => Self {
                dual_vdd_type : DualVddType::H2H,
            },
            "L" => Self {
                dual_vdd_type : DualVddType::L2H,
            },
            _ => unimplemented!()
        }
    }

    pub fn out_is_high(&self) -> bool {
        match self.dual_vdd_type {
            DualVddType::H2H => true,
            DualVddType::H2L(_) => false,
            DualVddType::L2H => true,
        }
    }

    pub fn is_h2h(&self) -> bool {
        match self.dual_vdd_type {
            DualVddType::H2H => true,
            _ => false,
        }
    }

    pub fn is_l2h(&self) -> bool {
        match self.dual_vdd_type {
            DualVddType::L2H => true,
            _ => false,
        }
    }
}
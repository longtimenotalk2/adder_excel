use std::collections::BTreeSet;

use crate::std::logic_block::Port;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CustomLogicBlock {
    NR4,
    NR6,
    AOI221,
    AOI2221,
}

impl CustomLogicBlock {
    pub fn ports_input_raw(&self) -> Vec<&'static str> {
        let ports: Vec<&'static str> = match self {
            Self::NR4 => vec!["A1", "A2", "A3", "A4"],
            Self::NR6 => vec!["A1", "A2", "A3", "A4", "A5", "A6"],
            Self::AOI221 => vec!["A1", "A2", "B1", "B2", "C"],
            Self::AOI2221 => vec!["A1", "A2", "B1", "B2", "C1", "C2", "D"],
        };
        ports
    }

    pub fn ports_input(&self) -> BTreeSet<Port> {
        self.ports_input_raw().iter().map(|p| Port(p.to_string())).collect()
    }

    pub fn ports_output_raw(&self) -> Vec<&'static str> {
        vec!["ZN"]
    }

    pub fn ports_output(&self) -> BTreeSet<Port> {
        self.ports_output_raw().iter().map(|p| Port(p.to_string())).collect()
    }
}
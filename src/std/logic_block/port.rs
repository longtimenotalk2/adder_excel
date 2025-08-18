use std::collections::BTreeSet;

use crate::std::logic_block::{LogicBlock, Port};

impl Port {
    pub fn new(s : &str) -> Self {
        Self(s.to_string())
    }
}

impl LogicBlock {
    pub fn ports_input(&self) -> BTreeSet<Port> {
        let ports = match self {
            Self::INV => vec!["I"],
            Self::ND2 | Self::NR2 | Self::AN2 | Self::OR2 |
                Self::XOR2 | Self::XNR2 | Self::XNR2DOUT | Self::XOR2DOUT
                => vec!["A1", "A2"],
            Self::IND2 | Self::INR2 => vec!["A1", "B1"], // A1 add INV
            Self::AOI21 | Self::OAI21 | Self::AO21 | Self::OA21 |
                Self::IAOI21 | Self::IOAI21 // A2 add INV
                => vec!["A1", "A2", "B"],
            Self::AOI22 | Self::OAI22 => vec!["A1", "A2", "B1", "B2"],
            Self::AOAI211 | Self::OAOI211 | Self::AOA211 | Self::OAO211
                => vec!["A1", "A2", "B", "C"],
        };
        ports.iter().map(|p| Port(p.to_string())).collect()
    }

    pub fn ports_output(&self) -> BTreeSet<Port> {
        let ports = match self {
            Self::XNR2DOUT => vec!["O1", "ZN"],
            Self::XOR2DOUT => vec!["O1", "Z"],
            Self::AN2 | Self::OR2 | Self::XOR2 |
                Self::AO21 | Self::OA21 | Self::AOA211 | Self::OAO211
                => vec!["Z"],
            _ => vec!["ZN"],
        };
        ports.iter().map(|p| Port(p.to_string())).collect()
    }
}
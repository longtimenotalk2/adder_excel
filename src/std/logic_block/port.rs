use std::collections::HashSet;

use crate::std::logic_block::{LogicBlock, Port};

impl LogicBlock {
    pub fn ports_input(&self) -> HashSet<Port> {
        let ports = match self {
            Self::INV => vec!["I"],
            Self::ND2 | Self::NR2 | Self::AN2 | Self::OR2 |
                Self::XOR2 | Self::XNR2 | Self::XNR2DOUT | Self::XOR2DOUT
                => vec!["A1", "A2"],
            Self::IND2 | Self::INR2 => vec!["A1", "B1"],
            Self::AOI21 | Self::OAI21 | Self::AO21 | Self::OA21 |
                Self::IAOI21 | Self::IOAI21
                => vec!["A1", "A2", "B"],
            Self::AOI22 | Self::OAI22 => vec!["A1", "A2", "B1", "B2"],
            Self::AOAI211 | Self::OAOI211 | Self::AOA211 | Self::OAO211
                => vec!["A1", "A2", "B", "C"],
        };

        ports.iter().map(|p| Port(p.to_string())).collect()

    }
}
use std::collections::HashSet;

use crate::std::logic_block::{LogicBlock, Port};

impl LogicBlock {
    pub fn ports_input(&self) -> HashSet<Port> {
        let ports = match self {
            Self::INV => vec!["I"],
            _ => todo!()
        };

        ports.iter().map(|p| Port(p.to_string())).collect()

    }
}
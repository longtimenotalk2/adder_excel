use std::{collections::BTreeMap, fmt::Debug};

use colorful::{Color, Colorful};

use crate::std::{logic_block::{LogicBlock, Port}, node_create::{LogicBlockCreateError, LogicBlockMappingTable}};

impl LogicBlockCreateError {
    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        ret += &format!("hint : {:?}\n", self.hint);
        ret += "founded : \n";
        for wire in &self.found_wires {
            ret += &format!("> {}\n", format!("{:?}", wire).color(Color::Green));
        }
        ret += "expected but not match: \n";
        ret += &format!("? {}", format!("{:?}", self.misfound_wire).color(Color::Red));
        ret
    }
}

impl Debug for LogicBlockCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl LogicBlockMappingTable {
    pub fn rev(&self) -> Self {
        let mut this = self.clone();
        this.logic_block = this.logic_block.rev();

        let mut new_outs = BTreeMap::new();

        match this.logic_block.clone() {
            LogicBlock::XNR2 | LogicBlock::XOR2 | LogicBlock::XNR2DOUT | LogicBlock::XOR2DOUT => {
                for (port, wire) in &this.outputs {
                    let port = if port.0 == "ZN" {
                        Port::new("Z")
                    } else if port.0 == "Z" {
                        Port::new("ZN")
                    } else {
                        port.clone()
                    };
                    new_outs.insert(port, wire.clone());
                }
            },
            _ => new_outs = this.outputs.clone(),
        }

        this.outputs = new_outs;

        this
    }
}

#[test]
fn test() {
    let mut k : BTreeMap<String, usize> = BTreeMap::new();

}
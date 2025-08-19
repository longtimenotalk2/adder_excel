use std::collections::BTreeMap;

use crate::std::{node_create::LogicBlockMappingTable, wire::Wire};

impl LogicBlockMappingTable {
    pub fn execute_with_signals(&self, signals : &mut BTreeMap<Wire, bool>) {
        let mut input_signals = BTreeMap::new();
        for (port, wire) in &self.inputs {
            let value = signals.get(wire).expect(&format!("wire {:?} not found", wire));
            input_signals.insert(port.clone(), *value);
        }
        let output_signals = self.logic_block.calc(input_signals);
        for (port, wire) in &self.outputs {
            if signals.contains_key(wire) {
                panic!("wire {:?} already exists", wire);
            }
            signals.insert(wire.clone(), *output_signals.get(port).expect(&format!("port {:?} not found", port)));
        }
    }
}
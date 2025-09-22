use std::collections::{BTreeMap, BTreeSet};

use crate::std::{adder::{AbstractCell, Adder, CellFullInfoInAdder}, logic_block::Port};

impl CellFullInfoInAdder {
    pub fn to_abstract_cell(&self) -> AbstractCell {
        AbstractCell {
            logic_block : self.logic_block_map.logic_block.clone(),
            drive : self.drive.clone(),
            custom_demand : self.custom_demand.clone(),
        }
    }

    pub fn inst_name(&self) -> String {
        let mut wire_name = String::new();
        for wire in self.logic_block_map.outputs.values() {
            wire_name.push_str(&wire.to_string());
        }
        format!("U{}_{}", self.layer, wire_name)
    }

    pub fn capi_input(&self) -> BTreeMap<Port, i32> {
        let mut ret = BTreeMap::new();

        for (port, cap) in &self.logic_block_map.logic_block.capi_input() {
            let cap = cap * self.drive.drive_num();
            ret.insert(port.clone(), cap);
        }

        ret
    }

    pub fn capi_output(&self) -> BTreeMap<Port, i32> {
        self.logic_block_map.logic_block.capi_output()
    }
}

impl Adder {
    pub fn all_abstract_cells(&self) -> BTreeSet<AbstractCell> {
        let mut set = BTreeSet::new();
        for info in &self.cells {
            set.insert(info.to_abstract_cell());
        }
        set
    }

    pub fn all_custom_abstract_cells(&self) -> BTreeSet<AbstractCell> {
        self.all_abstract_cells().iter().filter(|c| c.custom_demand.len() > 0).cloned().collect()
    }
}
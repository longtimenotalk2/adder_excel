use std::collections::BTreeSet;

use crate::std::adder::{AbstractCell, Adder, CellFullInfoInAdder};

impl CellFullInfoInAdder {
    pub fn to_abstract_cell(&self) -> AbstractCell {
        AbstractCell {
            logic_block : self.logic_block_map.logic_block.clone(),
            drive : self.drive.clone(),
            custom_demand : self.custom_demand.clone(),
        }
    }
}

impl Adder {
    pub fn all_custom_properties(&self) -> BTreeSet<AbstractCell> {
        let mut set = BTreeSet::new();
        for info in &self.cells {
            set.insert(info.to_abstract_cell());
        }
        set
    }
}
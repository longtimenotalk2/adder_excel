use std::collections::BTreeSet;

use crate::adder_v2::{adder::{Adder}, cell::CellBody};

impl Adder {
    pub fn cell_body_set(&self) -> BTreeSet<CellBody> {
        let mut ret = BTreeSet::new();
        for (_, cell) in self.cells.iter() {
            ret.insert(cell.to_cell_body());
        }
        ret
    }
}
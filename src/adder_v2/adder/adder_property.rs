use std::collections::BTreeSet;

use crate::adder_v2::{adder::Adder, cell::cell_body::CellBody};

impl Adder {
    pub fn cell_body_set(&self) -> BTreeSet<CellBody> {
        let mut ret = BTreeSet::new();
        for (_, cell) in self.cells.iter() {
            ret.insert(cell.to_cell_body());
        }
        ret
    }

    pub fn cell_num(&self) -> i32 {
        self.cells.len() as i32
    }

    pub fn mos_num(&self) -> i32 {
        self.cells.iter().map(|c| c.1.to_cell_body().mos_num()).sum()
    }
}
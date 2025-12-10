use std::collections::BTreeSet;

use crate::adder_v2::floorplan_m_v1::{CellId, FloorPlanMV1, M1XEnum, M1YRange, WireId};

impl M1YRange {
    pub fn new_single(y : i32) -> Self {
        Self { start: y, end: y }
    }

    pub fn new_range(start: i32, end: i32) -> Self {
        Self { start: start, end: end }
    }
}

impl M1XEnum {
    pub fn new(x : &[i32]) -> Self {
        let mut x_set = BTreeSet::new();
        for i in x {
            x_set.insert(*i);
        }
        Self(x_set)
    }
}

impl FloorPlanMV1 {
    pub fn find_cell_id_from_name(&self, name: &str) -> Option<CellId> {
        for (i, cell) in &self.cell_static_data {
            if cell.name == name {
                return Some(*i);
            }
        }
        None
    }

    pub fn find_wire_id_from_name(&self, name: &str) -> Option<WireId> {
        for (i, wire) in &self.wire_static_data {
            if wire.name == name {
                return Some(*i);
            }
        }
        None
    }
}


use std::ops::Range;

use crate::adder_v2::{node::{node_create::NodeCreateError, node_create_branch::WireRange}, wire::{FlagP, Wire}, Id};

pub struct WireList(Vec<(Id, Wire)>);

impl WireList {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn find(&self, wire: &Wire) -> Result<(Id, Wire), NodeCreateError> {
        let iter = self.0.iter();

        for (id, w) in iter {
            if wire.is_logic_equil(w) {
                return Ok((*id, w.clone()));
            }
        }

        Err(NodeCreateError::CanNotFindGivenWire(wire.clone()))
    }

    /// 寻找index的start和end可以浮动的flag extend
    pub fn find_wire_range(&self, wire_range : &WireRange) -> Vec<(Id, Wire)> {
        let iter = self.0.iter();

        let mut ret = vec![];

        for (id, w) in iter {
            if w.to_flag_p() == wire_range.to_flag_p() && wire_range.index_range.contains(&w.index) && wire_range.end_index_range.contains(&w.index_end()) {
                ret.push((*id, w.clone()));
            }
        }

        ret
    }
}
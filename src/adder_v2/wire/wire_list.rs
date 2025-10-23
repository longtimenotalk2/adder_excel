

use std::ops::Range;

use crate::adder_v2::{node::node_create::NodeCreateError, wire::{FlagExtend, Wire}, Id};

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
    pub fn find_ranged_start_and_end(&self, flag_extend : &FlagExtend, start_range : Range<usize>, end_range : Range<usize>) -> Vec<(Id, Wire)> {
        let iter = self.0.iter();

        let mut ret = vec![];

        for (id, w) in iter {
            if w.to_flag_extend() == *flag_extend && start_range.contains(&w.index) && end_range.contains(&w.index_end()) {
                ret.push((*id, w.clone()));
            }
        }

        ret
    }
}
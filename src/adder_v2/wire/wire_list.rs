

use std::ops::Range;

use crate::adder_v2::{node::{node_create::NodeCreateError, pure_logic_layer::WireRange}, wire::{FlagP, Wire}, Id};

pub struct WireList(pub Vec<(Id, Wire)>);

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

    pub fn find_and_replace(&mut self, wire: &Wire, wire_replaced: Wire) {

        for (_, w) in self.0.iter_mut() {
            if wire.is_logic_equil(w) {
                *w = wire_replaced;
                return;
            }
        }
    }

    /// 寻找index的start和end可以浮动的flag extend
    pub fn find_wire_range(&self, wire_range : &WireRange) -> Vec<(Id, Wire)> {
        let iter = self.0.iter();

        let mut ret = vec![];

        for (id, w) in iter {
            let to_wire = wire_range.try_to_wire(w.index, w.len);
            if let Some(wire) = to_wire {
                if wire.is_logic_equil(w) {
                    ret.push((*id, w.clone()));
                }
            }
        }

        ret
    }
}
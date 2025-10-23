use crate::adder_v2::{wire::Wire, Id};

pub struct WireList(Vec<(Id, Wire)>);

impl WireList {
    pub fn find(&self, wire: &Wire) -> Option<(Id, Wire)> {
        let iter = self.0.iter();

        for (id, w) in iter {
            if wire.is_logic_equil(w) {
                return Some((*id, w.clone()));
            }
        }

        None
    }
}
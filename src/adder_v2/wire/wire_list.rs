use crate::adder_v2::{node::node_create::NodeCreateError, wire::Wire, Id};

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
}
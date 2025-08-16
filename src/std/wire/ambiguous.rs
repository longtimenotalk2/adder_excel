use crate::std::wire::{AmbiguousWire, Wire};

impl Wire {
    pub fn to_ambiguous(self) -> AmbiguousWire {
        AmbiguousWire::Precise(self)
    }
}
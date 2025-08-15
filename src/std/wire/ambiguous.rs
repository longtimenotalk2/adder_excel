use crate::std::wire::{AmbiguousWire, Wire};

impl Wire {
    pub fn to_ambiguous(&self) -> AmbiguousWire {
        AmbiguousWire {
            flag : self.flag,
            is_neg : Some(self.is_neg),
            is_ab_xr : Some(self.is_ab_xr),
            index : self.index,
            len : Some(self.len),
        }
    }
}
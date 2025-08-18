use crate::std::wire::{Flag, Wire};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AmbiguousWire {
    Precise(Wire),
    MayLenNotGivenOrSearchMax { flag : Flag, is_neg : bool, index : usize, may_len : Option<usize> },
    LenGivenByOrder { flag : Flag, is_neg : bool, index : usize, len_choice : Vec<usize> },
}

impl Wire {
    pub fn to_ambiguous(self) -> AmbiguousWire {
        AmbiguousWire::Precise(self)
    }
}
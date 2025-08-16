mod basic;
mod ambiguous;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flag {
    G,
    P,
    H,
    AB,
    S,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Wire {
    flag : Flag,
    is_neg : bool,
    is_ab_xr : bool,
    index : usize,
    len : usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AmbiguousWire {
    Precise(Wire),
}
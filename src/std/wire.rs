mod basic;
mod ambiguous;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flag {
    G,
    P, // 最低bit是或逻辑
    Q, // 最低bit是异或逻辑
    H,
    A,
    B,
    S,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Wire {
    flag : Flag,
    is_neg : bool,
    index : usize,
    len : usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AmbiguousWire {
    Precise(Wire),
}
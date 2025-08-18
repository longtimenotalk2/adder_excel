mod basic;
pub mod ambiguous;

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
    pub flag : Flag,
    pub is_neg : bool,
    pub index : usize,
    pub len : usize,
}


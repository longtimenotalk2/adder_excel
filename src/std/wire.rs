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

pub struct AmbiguousWire {
    flag : Flag,  // 目前，所需flag类型都是定死的
    is_neg : Option<bool>,  // 最终异或运算时，不定死异或的极性
    is_ab_xr : Option<bool>,  // 部分p产生的逻辑，不在乎是异或的还是或的
    index : usize,  // 目前，所需index是定死的
    len : Option<usize>,  // 很多进位逻辑未定死选择多长的，此时选择一个最长的
}
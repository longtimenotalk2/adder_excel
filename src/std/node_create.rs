pub mod create_middle_node;
pub mod basic;
pub mod function;

use std::collections::{BTreeMap, BTreeSet};

use crate::std::{logic_block::{LogicBlock, Port}, wire::{ambiguous::AmbiguousWire, Flag, Wire}};

#[derive(Debug, Clone)]
pub enum LogiBlockHint {
    INV,
    Normal {
        flags : Vec<Flag>, // 从index大到index小的顺序的输入的flag
        is_out_inv : bool, // cell输出是否额外安插一个INV，如ND换AN
        custom_input_invs : BTreeSet<usize>, // 默认输入是同极性的，这里列的是输入极性相反的地址
        custom_input_lens : BTreeMap<usize, usize>, // 如果强制指定某个输入的长度，在这里指定
    },
    OnlyFromAB,
    XNRDOUT(bool), // is zn out
    XORDOUT(bool), // is z out
}

#[derive(Debug, Clone)]
pub struct LogicBlockMappingTable {
    pub logic_block : LogicBlock,
    pub inputs : BTreeMap<Port, Wire>,
    pub outputs : BTreeMap<Port, Wire>,
}

#[derive(Clone)]
pub struct LogicBlockCreateError {
    hint : LogiBlockHint,
    found_wires : Vec<Wire>,
    misfound_wire : AmbiguousWire,
}


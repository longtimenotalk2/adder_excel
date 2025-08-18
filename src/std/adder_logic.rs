mod create;

use std::collections::{BTreeMap, BTreeSet};

use crate::std::{logic_block::{LogicBlock, Port}, wire::{AmbiguousWire, Flag, Wire}};

#[derive(Debug, Clone)]
pub enum LogiBlockHint {
    INV,
    Normal {
        flags : Vec<Flag>,
        is_out_inv : bool,
        input_invs : BTreeSet<usize>,
    },
    OnlyFromAB,
    XNRDOUT(bool), // is zn out
    XORDOUT(bool), // is z out
}

pub struct LogicBlockMappingTable {
    logic_block : LogicBlock,
    inputs : BTreeMap<Port, Wire>,
    outputs : BTreeMap<Port, Wire>,
}

pub struct LogicBlockCreateError {
    hint : LogiBlockHint,
    found_wires : Vec<Wire>,
    misfound_wire : AmbiguousWire,
}


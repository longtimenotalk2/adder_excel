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
}

pub struct LogicBlockMappingTable {
    logic_block : LogicBlock,
    inputs : BTreeMap<Port, Wire>,
    outputs : BTreeMap<Port, Wire>,
}

impl LogicBlockMappingTable {
    // pub fn new(&self) -> &LogicBlock {
    //     // 需查验ports完备性
    // }
}

pub struct LogicBlockCreateError {
    hint : LogiBlockHint,
    found_wires : Vec<Wire>,
    misfound_wire : AmbiguousWire,
}

impl LogicBlockMappingTable {
    pub fn create_from_wire(target_wire : &Wire, hint : LogiBlockHint, history_wires : &[Wire]) -> Result<Self, LogicBlockCreateError> {
        // 如需寻找，则从后向前寻找
        let history_wires  : Vec<&Wire> = history_wires.iter().rev().collect();
        let mut found_wires = Vec::new();
        let error_return = |misfound_wire : AmbiguousWire| -> LogicBlockCreateError{
            LogicBlockCreateError {
                hint : hint.clone(),
                found_wires,
                misfound_wire,
            }
        };
        match &hint {
            LogiBlockHint::INV => {
                let needed_wire = target_wire.rev();
                if !history_wires.contains(&&needed_wire) {
                    return Err(error_return(needed_wire.to_ambiguous()));
                }
                todo!()
                // Ok(LogicBlockMappingTable{
                //     logic_block : LogicBlock::INV,
                //     inputs : BTreeMap::from([(Port::A, needed_wire.rev())]),
                // })
            }
            _ => todo!()
        }
    }
}
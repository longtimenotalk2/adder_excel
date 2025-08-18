use std::collections::{BTreeMap, BTreeSet};

use crate::std::{adder_logic::{LogiBlockHint, LogicBlockCreateError, LogicBlockMappingTable}, logic_block::{LogicBlock, Port}, wire::{AmbiguousWire, Flag, Wire}};

impl LogicBlockMappingTable {
    pub fn new(logic_block : LogicBlock, inputs : BTreeMap<Port, Wire>, outputs : BTreeMap<Port, Wire>) -> Self {
        let collect : BTreeSet<Port> = inputs.keys().into_iter().map(|s| s.clone()).collect();
        if logic_block.ports_input() != collect {
            panic!("logic block {logic_block:?} need input ports {:?}, but got {:?}", logic_block.ports_input(), collect);
        }
        let collect : BTreeSet<Port> = outputs.keys().into_iter().map(|s| s.clone()).collect();
        if logic_block.ports_output() != collect {
            panic!("logic block {logic_block:?} need output ports {:?}, but got {:?}", logic_block.ports_output(), collect);
        }
        Self {
            logic_block,
            inputs,
            outputs,
        }
    }
}

impl LogicBlockMappingTable {
    pub fn create_from_wire_by_hint(target_wire : &Wire, hint : LogiBlockHint, history_wires : &[Wire]) -> Result<Self, LogicBlockCreateError> {
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
                    Err(error_return(needed_wire.to_ambiguous()))
                } else {
                    Ok({Self::new(
                        LogicBlock::INV, 
                        BTreeMap::from([(Port::new("I"), needed_wire)]), 
                        BTreeMap::from([(Port::new("ZN"), target_wire.clone())])
                    )})
                }
            }
            LogiBlockHint::OnlyFromAB => {
                match target_wire.flag {
                    Flag::G => {
                        assert_eq!(target_wire.len, 1);
                        todo!()
                    },
                    _ => todo!()
                }
            }
            _ => todo!()
        }
    }
}
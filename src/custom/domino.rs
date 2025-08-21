use std::collections::BTreeMap;

use crate::{custom::custom_logic_block::CustomLogicBlock, std::{logic_block::{LogicBlock, Port}, node_create::{create_middle_node::WireManager, LogiBlockHint, LogicBlockCreateError, LogicBlockMappingTable}, wire::{ambiguous::AmbiguousWire, Flag, Wire}}};

#[derive(Debug, Clone)]
pub enum DominoPolar {
    P,
    N,
}

#[derive(Debug, Clone)]
pub struct DominoDemand {
    pub logic_block : LogicBlock,
    pub polar : DominoPolar,
    pub is_start : bool,
}

impl DominoDemand {
    pub fn from_strings(ss : &[String]) -> DominoDemand {
        assert_eq!(ss[0], "DOM");
        let logic_block = match ss[1].as_str() {
            "AOI21" => LogicBlock::AOI21,
            "OAI21" => LogicBlock::OAI21,
            "NR2" => LogicBlock::NR2,
            "NR4" => LogicBlock::Custom(CustomLogicBlock::NR4),
            "NR6" => LogicBlock::Custom(CustomLogicBlock::NR6),
            "AOI221" => LogicBlock::Custom(CustomLogicBlock::AOI221),
            "AOI2221" => LogicBlock::Custom(CustomLogicBlock::AOI2221),
            _ => panic!("Unknown logic block {}", ss[1]),
        };
        let polar = match ss[2].as_str() {
            "P" => DominoPolar::P,
            "N" => DominoPolar::N,
            _ => panic!("Unknown polar {}", ss[2]),
        };
        let is_start = if let Some(start_note) = ss.get(3) {
            assert_eq!(start_note, "S");
            true
        } else {
            false
        };
        DominoDemand {
            logic_block,
            polar,
            is_start,
        }
    }

    pub fn create_logic_block_mapping_table(
        &self, 
        target_wire : &Wire,
        manager : &mut WireManager,
        hint : &LogiBlockHint,
    ) -> Result<LogicBlockMappingTable, LogicBlockCreateError> {
        match hint {
            LogiBlockHint::Normal { flags, is_out_inv, custom_input_invs, custom_input_lens } => {
                if flags == &vec![Flag::P; 4] {
                    assert_eq!(self.logic_block, LogicBlock::Custom(CustomLogicBlock::NR4));
                    assert_eq!(target_wire.is_neg, false);
                    let mut input_wires = vec![];
                    let mut index_now = target_wire.index;
                    for _ in 0..3 {
                        let wire_need = AmbiguousWire::MayLenNotGivenOrSearchMax  {
                            flag: Flag::P,
                            index: index_now,
                            may_len: None,
                            is_neg: !target_wire.is_neg,
                        };
                        let wire = manager.find(&wire_need)?;
                        index_now -= wire.len;
                        input_wires.push(wire);
                    }
                    let wire_need = AmbiguousWire::Precise( Wire  {
                        flag: Flag::P,
                        index: index_now,
                        len: target_wire.len + index_now - target_wire.index,
                        is_neg: !target_wire.is_neg,
                    });
                    let wire = manager.find(&wire_need)?;
                    input_wires.push(wire);
                    Ok(LogicBlockMappingTable::new_from_vec(
                        self.logic_block.clone(),
                        input_wires,
                        vec![target_wire.clone()],
                    ))
                } else if flags == &vec![Flag::P; 6] {
                    assert_eq!(self.logic_block, LogicBlock::Custom(CustomLogicBlock::NR6));
                    assert_eq!(target_wire.is_neg, false);
                    let mut input_wires = vec![];
                    let mut index_now = target_wire.index;
                    for _ in 0..5 {
                        let wire_need = AmbiguousWire::MayLenNotGivenOrSearchMax  {
                            flag: Flag::P,
                            index: index_now,
                            may_len: None,
                            is_neg: !target_wire.is_neg,
                        };
                        let wire = manager.find(&wire_need)?;
                        index_now -= wire.len;
                        input_wires.push(wire);
                    }
                    let wire_need = AmbiguousWire::Precise( Wire  {
                        flag: Flag::P,
                        index: index_now,
                        len: target_wire.len + index_now - target_wire.index,
                        is_neg: !target_wire.is_neg,
                    });
                    let wire = manager.find(&wire_need)?;
                    input_wires.push(wire);
                    Ok(LogicBlockMappingTable::new_from_vec(
                        self.logic_block.clone(),
                        input_wires,
                        vec![target_wire.clone()],
                    ))
                } else if flags == &[vec![Flag::H; 4], vec![Flag::P; 3]].concat() {
                    assert_eq!(self.logic_block, LogicBlock::Custom(CustomLogicBlock::AOI2221));
                    assert_eq!(target_wire.is_neg, true);

                    let mut inputs : BTreeMap<Port, Wire> = BTreeMap::new();

                    let port_order = [Port::new("D"), Port::new("C1"), Port::new("C2"), Port::new("B1"), Port::new("B2"), Port::new("A1"), Port::new("A2")];
                    let mut index_now = target_wire.index;

                    for (i, port) in port_order.iter().enumerate() {
                        let wire_need = if i == port_order.len() - 1 {
                            AmbiguousWire::Precise (Wire  {
                                flag: Flag::H,
                                index: index_now,
                                len: target_wire.len + index_now - target_wire.index,
                                is_neg: !target_wire.is_neg,
                            })
                            
                        } else if i % 2 == 0 {
                            AmbiguousWire::MayLenNotGivenOrSearchMax  {
                                flag: Flag::H,
                                index: index_now,
                                may_len: None,
                                is_neg: !target_wire.is_neg,
                            }
                        } else {
                            AmbiguousWire::Precise (Wire  {
                                flag: Flag::P,
                                index: target_wire.index - 1,
                                len: target_wire.index - index_now,
                                is_neg: !target_wire.is_neg,
                            })
                        };
                        let wire = manager.find(&wire_need)?;
                        if i != port_order.len() - 1 && i % 2 == 0 {
                            index_now -= wire.len;
                        }
                        inputs.insert(port.clone(), wire);
                    }
                    Ok(LogicBlockMappingTable::new(
                        self.logic_block.clone(),
                        inputs,
                        BTreeMap::from([(Port::new("ZN"), target_wire.clone())]),
                    ))
                } else if flags.len() > 3 {
                    panic!("{:?} logic for domino is not valid", flags)
                } else {
                    let result = LogicBlockMappingTable::create_logic_block_mapping_table(target_wire, manager, hint)?;
                    assert_eq!(result.logic_block, self.logic_block);
                    Ok(result)
                }
            }
            _ => unimplemented!()
        }
    }
}
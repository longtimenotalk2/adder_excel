use std::collections::{BTreeMap, BTreeSet};

use crate::std::{adder_logic::{LogiBlockHint, LogicBlockCreateError, LogicBlockMappingTable}, logic_block::{LogicBlock, Port}, wire::{ambiguous::AmbiguousWire, Flag, Wire}};

struct WireSet(Vec<Wire>);

impl WireSet {
    pub fn find(&self, ambiguous_wire : &AmbiguousWire) -> Option<Wire> {
        match ambiguous_wire {
            AmbiguousWire::Precise(wire) => {
                if self.0.contains(wire) {
                    Some(wire.clone())
                } else {
                    None
                }
            }
            AmbiguousWire::MayLenNotGivenOrSearchMax { flag, is_neg, index, may_len } => {
                if let Some(len) = may_len {
                    self.find(&AmbiguousWire::Precise(Wire::new(flag.clone(), *is_neg, *index, *len)))
                } else {
                    let mut match_list = vec![];
                    for wire in &self.0 {
                        if &wire.flag == flag && wire.is_neg == *is_neg && wire.index == *index {
                            match_list.push(wire.clone());
                        }
                    }
                    match_list.sort_by(|a, b| b.len.cmp(&a.len));
                    match_list.get(0).cloned()
                }
            }
        }
    }
}

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

    pub fn new_from_vec(logic_block : LogicBlock, inputs : Vec<Wire>, outputs : Vec<Wire>) -> Self {
        assert_eq!(inputs.len(), logic_block.ports_input().len());
        assert_eq!(outputs.len(), logic_block.ports_output().len());
        let mut inputs_dict = BTreeMap::new();
        let mut outputs_dict = BTreeMap::new();
        for (i, port) in logic_block.ports_input().iter().enumerate() {
            inputs_dict.insert(port.clone(), inputs[i].clone());
        }
        for (i, port) in logic_block.ports_output().iter().enumerate() {
            outputs_dict.insert(port.clone(), outputs[i].clone());
        }
        Self::new(
            logic_block,
            inputs_dict,
            outputs_dict,
        )
    }

    pub fn new_ind_inr_origin_is_or(out : Wire, a1 : Wire, a2 : Wire, a1_is_neg : bool, a2_is_neg : bool, out_is_neg : bool) -> Self {
        assert_ne!(a1_is_neg, a2_is_neg);
        match (a1_is_neg, a2_is_neg, out_is_neg) {
            (true, false, false) => {
                Self::new_from_vec(
                    LogicBlock::INR2,
                    vec![a1, a2],
                    vec![out],
                )
            },
            (false, true, false) => {
                Self::new_from_vec(
                    LogicBlock::INR2,
                    vec![a2, a1],
                    vec![out],
                )
            },
            (true, false, true) => {
                Self::new_from_vec(
                    LogicBlock::IND2,
                    vec![a2, a1],
                    vec![out],
                )
            },
            (false, true, true) => {
                Self::new_from_vec(
                    LogicBlock::IND2,
                    vec![a1, a2],
                    vec![out],
                )
            },
            _ => unimplemented!()
        }
    }
}

impl LogicBlockMappingTable {
    pub fn create_from_wire_by_hint(target_wire : &Wire, hint : LogiBlockHint, history_wires : &[Wire]) -> Result<Self, LogicBlockCreateError> {
        // 如需寻找，则从后向前寻找
        let history_wires  : Vec<Wire> = history_wires.iter().rev().cloned().collect();
        let history_wires = WireSet(history_wires);
        
        struct WireManager {
            hint : LogiBlockHint,
            found_wires : Vec<Wire>,
            history_wires : WireSet,
        }

        impl WireManager {
            fn find(&mut self, ambiguous_wire : &AmbiguousWire) -> Result<Wire, LogicBlockCreateError> {
                if let Some(wire) = self.history_wires.find(ambiguous_wire) {
                    self.found_wires.push(wire.clone());
                    Ok(wire)
                } else {
                    Err(LogicBlockCreateError{
                        hint : self.hint.clone(),
                        misfound_wire : ambiguous_wire.clone(),
                        found_wires : self.found_wires.clone(),
                    })
                }
            }
        }

        let mut manager = WireManager {
            hint : hint.clone(),
            found_wires : vec![],
            history_wires,
        };

        match &hint {
            LogiBlockHint::INV => {
                let needed_wire = manager.find(&AmbiguousWire::Precise(target_wire.rev()))?;
                Ok({Self::new_from_vec(
                    LogicBlock::INV, 
                    vec![needed_wire], 
                    vec![target_wire.clone()],
                )})
            }
            // OnlyFromAB 只接受G、P、Q、H四种选择，H的len必须为2，G、Q的len必须为1，P的len可以是1或者2。
            LogiBlockHint::OnlyFromAB => {
                match target_wire.flag {
                    Flag::G => {
                        assert_eq!(target_wire.len, 1);
                        let index = target_wire.index;
                        let is_out_inv = !target_wire.is_neg;
                        let logic_block = LogicBlock::ND2.if_add_out_inv(is_out_inv);
                        Ok(Self::new_from_vec(
                            logic_block, 
                            vec![Wire::from_str(&format!("a{index}")), Wire::from_str(&format!("b{index}"))],
                            vec![Wire::from_str(&format!("ng{index}")).if_rev(is_out_inv)]
                        ))
                    },
                    Flag::P => {
                        match target_wire.len {
                            1 => {
                                let index = target_wire.index;
                                let is_out_inv = !target_wire.is_neg;
                                let logic_block = LogicBlock::NR2.if_add_out_inv(is_out_inv);
                                Ok(Self::new_from_vec(
                                    logic_block, 
                                    vec![Wire::from_str(&format!("a{index}")), Wire::from_str(&format!("b{index}"))],
                                    vec![Wire::from_str(&format!("np{index}")).if_rev(is_out_inv)]
                                ))
                            },
                            2 => {
                                let index = target_wire.index;
                                assert_eq!(target_wire.is_neg, true);
                                let logic_block = LogicBlock::OAI22;
                                Ok(Self::new_from_vec(
                                    logic_block, 
                                    vec![Wire::from_str(&format!("a{index}")), Wire::from_str(&format!("b{index}")), Wire::from_str(&format!("a{}", index - 1)), Wire::from_str(&format!("b{}", index - 1))],
                                    vec![Wire::from_str(&format!("np{index}_{}", index - 1))]
                                ))
                            },
                            _ => panic!(),
                        }
                    },
                    Flag::H => {
                        assert_eq!(target_wire.len, 2);
                        let index = target_wire.index;
                        let logic_block = LogicBlock::AOI22;
                        Ok(Self::new_from_vec(
                            logic_block, 
                            vec![Wire::from_str(&format!("a{index}")), Wire::from_str(&format!("b{index}")), Wire::from_str(&format!("a{}", index - 1)), Wire::from_str(&format!("b{}", index - 1))],
                            vec![Wire::from_str(&format!("nh{index}_{}", index - 1))]
                        ))
                    },
                    Flag::Q => {
                        assert_eq!(target_wire.len, 1);
                        let index = target_wire.index;
                        let is_out_inv = !target_wire.is_neg;
                        let logic_block = LogicBlock::XNR2.if_add_out_inv(is_out_inv);
                        Ok(Self::new_from_vec(
                            logic_block, 
                            vec![Wire::from_str(&format!("a{index}")), Wire::from_str(&format!("b{index}"))],
                            vec![Wire::from_str(&format!("nq{index}")).if_rev(is_out_inv)]
                        ))
                    },
                    _ => panic!("Flag {:?} can not create by ab", target_wire.flag)
                }
            }
            LogiBlockHint::XNRDOUT(is_out_s) => {
                let index = target_wire.index;
                Ok(Self::new_from_vec(
                    LogicBlock::XNR2DOUT, 
                    vec![Wire::from_str(&format!("a{index}")), Wire::from_str(&format!("b{index}"))],
                    vec![Wire::from_str(&format!("ng{index}")), if *is_out_s {Wire::from_str(&format!("ns{index}"))} else {Wire::from_str(&format!("nq{index}"))}]
                ))
            }
            LogiBlockHint::XORDOUT(is_out_s) => {
                let index = target_wire.index;
                Ok(Self::new_from_vec(
                    LogicBlock::XNR2DOUT, 
                    vec![Wire::from_str(&format!("a{index}")), Wire::from_str(&format!("b{index}"))],
                    vec![Wire::from_str(&format!("np{index}")), if *is_out_s {Wire::from_str(&format!("n{index}"))} else {Wire::from_str(&format!("q{index}"))}]
                ))
            }
            LogiBlockHint::Normal { flags, is_out_inv, custom_input_invs, custom_input_lens } => {
                match target_wire.flag {
                    Flag::G => {
                        // 合成G的方式：
                        // 2输入，这种一定是P和H的组合，P长度为1，H的长度为目标长度
                        // 3输入，第一个默认找最长的，第二个数和第一个等长，第三个数计算具体长度
                        // 4输入
                        match flags.len() {
                            2 => {
                                assert_eq!(&flags[0], &Flag::P);
                                let source_p_is_neg = !target_wire.is_neg ^ is_out_inv ^ custom_input_invs.contains(&0);
                                let source_h_is_neg = !target_wire.is_neg ^ is_out_inv ^ custom_input_invs.contains(&1);
                                let source_p = manager.find(&AmbiguousWire::Precise(
                                    Wire::from_str_index_len("p", target_wire.index, 1).if_rev(source_p_is_neg)
                                ))?;
                                let source_h = manager.find(&AmbiguousWire::Precise(
                                    Wire::from_str_index_len("h", target_wire.index, target_wire.len).if_rev(source_h_is_neg)
                                ))?;

                                let logic_block_base = LogicBlock::NR2;
                                
                                if source_p_is_neg == source_h_is_neg {
                                    let logic_block = logic_block_base.if_rev(source_p_is_neg).if_add_out_inv(*is_out_inv);
                                    Ok(Self::new_from_vec(
                                        logic_block, 
                                        vec![source_p, source_h],
                                        vec![target_wire.clone()]
                                    ))
                                } else {
                                    Ok(Self::new_ind_inr_origin_is_or(
                                        target_wire.clone(), 
                                        source_p, 
                                        source_h, 
                                        source_p_is_neg, 
                                        source_h_is_neg, 
                                        target_wire.is_neg
                                    ))
                                }
                            },
                            3 => {
                                todo!()
                            }
                            _ => {unimplemented!()}
                        }
                    },
                    _ => todo!(),
                }
            }
        }
    }
}
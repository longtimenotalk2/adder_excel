use std::collections::{BTreeMap, BTreeSet};

use crate::{custom::domino::DominoDemand, std::{adder::CustomDemand, logic_block::{self, LogicBlock, Port}, node_create::{LogiBlockHint, LogicBlockCreateError, LogicBlockMappingTable}, wire::{ambiguous::AmbiguousWire, Flag, Wire}}};

pub struct WireSet(Vec<Wire>);

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
            AmbiguousWire::LenGivenByOrder { flag, is_neg, index, len_choice } => {
                for len in len_choice {
                    if let Some(wire) = self.find(&AmbiguousWire::Precise(Wire::new(flag.clone(), *is_neg, *index, *len))) {
                        return Some(wire);
                    }
                }
                None
            }
        }
    }
}

pub struct WireManager {
    pub hint : LogiBlockHint,
    pub found_wires : Vec<Wire>,
    pub history_wires : WireSet,
}

impl WireManager {
    pub fn find(&mut self, ambiguous_wire : &AmbiguousWire) -> Result<Wire, LogicBlockCreateError> {
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

    pub fn new_ind_inr_origin_is_and(out : Wire, a1 : Wire, a2 : Wire, a1_is_neg : bool, a2_is_neg : bool, out_is_neg : bool) -> Self {
        assert_ne!(a1_is_neg, a2_is_neg);
        match (a1_is_neg, a2_is_neg, out_is_neg) {
            (true, false, true) => {
                Self::new_from_vec(
                    LogicBlock::IND2,
                    vec![a1, a2],
                    vec![out],
                )
            },
            (false, true, true) => {
                Self::new_from_vec(
                    LogicBlock::IND2,
                    vec![a2, a1],
                    vec![out],
                )
            },
            (true, false, false) => {
                Self::new_from_vec(
                    LogicBlock::INR2,
                    vec![a2, a1],
                    vec![out],
                )
            },
            (false, true, false) => {
                Self::new_from_vec(
                    LogicBlock::INR2,
                    vec![a1, a2],
                    vec![out],
                )
            },
            _ => unimplemented!()
        }
    }

    pub fn new_aoi21_like(logic_block : LogicBlock, out : Wire, a1 : Wire, a2 : Wire, b : Wire, a1_is_neg : bool, a2_is_neg : bool, b_is_neg : bool, out_is_neg : bool) -> Self {
        assert_eq!(b_is_neg, false);
        assert!([LogicBlock::AOI21, LogicBlock::OAI21].contains(&logic_block));
        if a1_is_neg != a2_is_neg {
            let logic_block = logic_block.aoi21_like_input_inv();
            assert_eq!(out_is_neg, false);
            if a2_is_neg {
                Self::new_from_vec(
                    logic_block,
                    vec![a1, a2, b],
                    vec![out],
                )
            } else {
                Self::new_from_vec(
                    logic_block,
                    vec![a2, a1, b],
                    vec![out],
                )
            }
        } else {
            if out_is_neg {
                Self::new_from_vec(
                    logic_block.if_add_out_inv(true),
                    vec![a1, a2, b],
                    vec![out],
                )
            } else {
                Self::new_from_vec(
                    logic_block,
                    vec![a1, a2, b],
                    vec![out],
                )
            }
        }
        
    }
}

impl LogicBlockMappingTable {
    pub fn create_from_wire_by_hint_and_custom_demand(target_wire : &Wire, hint : &LogiBlockHint, history_wires : &[Wire], custom_demand : &[CustomDemand]) -> Result<Self, LogicBlockCreateError> {
        // 如需寻找，则从后向前寻找
        let history_wires  : Vec<Wire> = history_wires.iter().rev().cloned().collect();
        let history_wires = WireSet(history_wires);
        
        let mut manager = WireManager {
            hint : hint.clone(),
            found_wires : vec![],
            history_wires,
        };

        if custom_demand.len() > 0 {
            assert_eq!(custom_demand.len(), 1);
            let custom_demand = custom_demand[0].clone();
            match custom_demand {
                CustomDemand::Domino(domino_demand) => {
                    return domino_demand.create_logic_block_mapping_table(target_wire, &mut manager, hint)
                }
                _ => (),
            }
        }

        Self::create_logic_block_mapping_table(target_wire, &mut manager, hint)
    }

    pub fn create_logic_block_mapping_table(
        target_wire : &Wire,
        manager : &mut WireManager,
        hint : &LogiBlockHint
    ) -> Result<Self, LogicBlockCreateError> {
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
                        let logic_block = match target_wire.is_neg {
                            true => LogicBlock::XNR2,
                            false => LogicBlock::XOR2,
                        };
                        Ok(Self::new_from_vec(
                            logic_block, 
                            vec![Wire::from_str(&format!("a{index}")), Wire::from_str(&format!("b{index}"))],
                            vec![target_wire.clone()]
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
                    vec![Wire::from_str(&format!("ng{index}")), if *is_out_s {Wire::from_str(&format!("s{index}"))} else {Wire::from_str(&format!("nq{index}"))}]
                ))
            }
            LogiBlockHint::XORDOUT(is_out_s) => {
                let index = target_wire.index;
                Ok(Self::new_from_vec(
                    LogicBlock::XOR2DOUT, 
                    vec![Wire::from_str(&format!("a{index}")), Wire::from_str(&format!("b{index}"))],
                    vec![Wire::from_str(&format!("np{index}")), if *is_out_s {Wire::from_str(&format!("s{index}"))} else {Wire::from_str(&format!("q{index}"))}]
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
                                assert_eq!(&flags[1], &Flag::H);
                                let source_p_is_neg = !target_wire.is_neg ^ is_out_inv ^ custom_input_invs.contains(&0);
                                let source_h_is_neg = !target_wire.is_neg ^ is_out_inv ^ custom_input_invs.contains(&1);
                                let source_p = manager.find(&AmbiguousWire::Precise(
                                    Wire::from_str_index_len("p", target_wire.index, 1).if_rev(source_p_is_neg)
                                ))?;
                                let source_h = manager.find(&AmbiguousWire::Precise(
                                    Wire::from_str_index_len("h", target_wire.index, target_wire.len).if_rev(source_h_is_neg)
                                ))?;

                                let logic_block_base = LogicBlock::ND2;
                                
                                if source_p_is_neg == source_h_is_neg {
                                    let logic_block = logic_block_base.if_rev(source_p_is_neg).if_add_out_inv(*is_out_inv);
                                    Ok(Self::new_from_vec(
                                        logic_block, 
                                        vec![source_p, source_h],
                                        vec![target_wire.clone()]
                                    ))
                                } else {
                                    Ok(Self::new_ind_inr_origin_is_and(
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
                                assert_eq!(&flags[0], &Flag::G);
                                let logic_block = LogicBlock::AOI21.if_rev(!target_wire.is_neg ^ is_out_inv);
                                let source_first_g_is_neg = !target_wire.is_neg ^ is_out_inv ^ custom_input_invs.contains(&0);
                                let source_first_g = manager.find(&AmbiguousWire::MayLenNotGivenOrSearchMax { 
                                    flag: Flag::G, 
                                    is_neg: source_first_g_is_neg, 
                                    index: target_wire.index, 
                                    may_len: custom_input_lens.get(&0).copied(), 
                                })?;
                                let source_first_g_len = source_first_g.len;

                                match (&flags[1], &flags[2]) {
                                    (Flag::P, Flag::G) | (Flag::Q, Flag::G) | (Flag::P, Flag::H) => {
                                        let source_pq_is_neg = !target_wire.is_neg ^ is_out_inv ^ custom_input_invs.contains(&1);
                                        let source_pq_index = target_wire.index;
                                        let source_pg_len_shoice = if &flags[2] == &Flag::G && &flags[1] == &Flag::P {
                                            vec![source_first_g_len, source_first_g_len+1]
                                        } else if &flags[2] == &Flag::H {
                                            vec![source_first_g_len+1]
                                        } else {
                                            vec![source_first_g_len]
                                        };
                                        let source_pq = manager.find(&AmbiguousWire::LenGivenByOrder  {
                                                flag: flags[1],
                                                index: source_pq_index,
                                                len_choice: source_pg_len_shoice,
                                                is_neg: source_pq_is_neg,
                                            
                                        })?;
                                        let source_end_g_is_neg = !target_wire.is_neg ^ is_out_inv ^ custom_input_invs.contains(&2);
                                        let source_end_g_index = target_wire.index - source_first_g_len;
                                        let source_end_g_len = target_wire.len - source_first_g_len;
                                        let source_end_g = manager.find(&AmbiguousWire::Precise (
                                            Wire {
                                                flag: flags[2],
                                                index: source_end_g_index,
                                                len: source_end_g_len,
                                                is_neg: source_end_g_is_neg,
                                            }
                                        ))?;
                                        Ok(Self::new_aoi21_like(logic_block, target_wire.clone(), 
                                            source_pq, source_end_g, source_first_g, 
                                            custom_input_invs.contains(&1), custom_input_invs.contains(&2), custom_input_invs.contains(&0), 
                                            *is_out_inv
                                        ))
                                    },
                                    _ => panic!("3 input gen G, index 1 and 2 flag can't be {:?}, {:?}", &flags[1],&flags[2])
                                }
                            }
                            4 => {
                                if &flags[0] == &Flag::P {
                                    assert_eq!(&flags[1], &Flag::H);
                                    assert_eq!(custom_input_invs.len(), 0);
                                    let input_is_neg = !target_wire.is_neg ^ is_out_inv;
                                    let source_this_p = manager.find(&AmbiguousWire::Precise(
                                        Wire {
                                            flag: Flag::P,
                                            index: target_wire.index,
                                            len: 1,
                                            is_neg: input_is_neg,
                                        }
                                    ))?;
                                    let source_first_h = manager.find(&AmbiguousWire::MayLenNotGivenOrSearchMax {
                                        flag : Flag::H,
                                        is_neg: input_is_neg,
                                        index: target_wire.index,
                                        may_len: custom_input_lens.get(&1).copied(),
                                    })?;
                                    let source_first_h_len = source_first_h.len;
                                    let source_pq_index = target_wire.index - 1;
                                    let source_pq_len_shoice = if &flags[3] == &Flag::G && &flags[2] == &Flag::P {
                                        vec![source_first_h_len-1, source_first_h_len]
                                    } else if &flags[3] == &Flag::H {
                                        vec![source_first_h_len]
                                    } else {
                                        vec![source_first_h_len-1]
                                    };
                                    let source_pq = manager.find(&AmbiguousWire::LenGivenByOrder  {
                                        flag: flags[2],
                                        index: source_pq_index,
                                        len_choice: source_pq_len_shoice,
                                        is_neg: input_is_neg,
                                    })?;
                                    let source_end_g_index = target_wire.index - source_first_h_len;
                                    let source_end_g_len = target_wire.len - source_first_h_len;
                                    let source_end_g = manager.find(&AmbiguousWire::Precise (
                                        Wire {
                                            flag: flags[3],
                                            index: source_end_g_index,
                                            len: source_end_g_len,
                                            is_neg: input_is_neg,
                                        }
                                    ))?;

                                    Ok(Self::new_from_vec(
                                        LogicBlock::AOAI211.if_rev(input_is_neg).if_add_out_inv(*is_out_inv), 
                                        vec![source_end_g, source_pq, source_first_h, source_this_p], 
                                        vec![target_wire.clone()], 
                                    ))
                                } else if &flags[0] == &Flag::A {
                                    // 从AB直接出
                                    assert_eq!(&flags[1], &Flag::B);
                                    let source_pq_index = target_wire.index;
                                    let source_pq_len_shoice = if &flags[3] == &Flag::G && &flags[2] == &Flag::P {
                                        vec![1, 2]
                                    } else if &flags[3] == &Flag::H {
                                        vec![2]
                                    } else {
                                        vec![1]
                                    };
                                    let source_pq = manager.find(&AmbiguousWire::LenGivenByOrder  {
                                        flag: flags[2],
                                        index: source_pq_index,
                                        len_choice: source_pq_len_shoice,
                                        is_neg: false,
                                    })?;
                                    let source_end_g_index = target_wire.index - 1;
                                    let source_end_g_len = target_wire.len - 1;
                                    let source_end_g = manager.find(&AmbiguousWire::Precise (
                                        Wire {
                                            flag: flags[3],
                                            index: source_end_g_index,
                                            len: source_end_g_len,
                                            is_neg: false,
                                        }
                                    ))?;
                                    Ok(Self::new_from_vec(
                                        LogicBlock::AOI22, 
                                        vec![
                                            Wire::from_str(&format!("a{}",target_wire.index)), 
                                            Wire::from_str(&format!("b{}",target_wire.index)), 
                                            source_pq,
                                            source_end_g,
                                        ], 
                                        vec![target_wire.clone()], 
                                    ))

                                } else if &flags[0] == &Flag::H {
                                    // 用AOI22逻辑，先在原位把h加工成g
                                    assert_eq!(&flags[1], &Flag::P);
                                    assert_eq!(custom_input_invs.len(), 0);
                                    let input_is_neg = !target_wire.is_neg ^ is_out_inv;
                                    let source_this_p = manager.find(&AmbiguousWire::Precise(
                                        Wire {
                                            flag: Flag::P,
                                            index: target_wire.index,
                                            len: 1,
                                            is_neg: input_is_neg,
                                        }
                                    ))?;
                                    let source_first_h = manager.find(&AmbiguousWire::MayLenNotGivenOrSearchMax {
                                        flag : Flag::H,
                                        is_neg: input_is_neg,
                                        index: target_wire.index,
                                        may_len: custom_input_lens.get(&1).copied(),
                                    })?;
                                    let source_first_h_len = source_first_h.len;
                                    let source_pq_index = target_wire.index;
                                    let source_pq_len_shoice = if &flags[3] == &Flag::G && &flags[2] == &Flag::P {
                                        vec![source_first_h_len, source_first_h_len+1]
                                    } else if &flags[3] == &Flag::H {
                                        vec![source_first_h_len+1]
                                    } else {
                                        vec![source_first_h_len]
                                    };
                                    let source_pq = manager.find(&AmbiguousWire::LenGivenByOrder  {
                                        flag: flags[2],
                                        index: source_pq_index,
                                        len_choice: source_pq_len_shoice,
                                        is_neg: input_is_neg,
                                    })?;
                                    let source_end_g_index = target_wire.index - source_first_h_len;
                                    let source_end_g_len = target_wire.len - source_first_h_len;
                                    let source_end_g = manager.find(&AmbiguousWire::Precise (
                                        Wire {
                                            flag: flags[3],
                                            index: source_end_g_index,
                                            len: source_end_g_len,
                                            is_neg: input_is_neg,
                                        }
                                    ))?;

                                    Ok(Self::new_from_vec(
                                        LogicBlock::AOI22.if_rev(input_is_neg).if_add_out_inv(*is_out_inv), 
                                        vec![source_end_g, source_pq, source_first_h, source_this_p], 
                                        vec![target_wire.clone()], 
                                    ))
                                } else {
                                    unimplemented!()
                                }
                                
                            }
                            _ => {unimplemented!()}
                        }
                    },
                    Flag::H => {
                        // 合成G的方式：
                        
                        if flags.len() == 3 {
                            // 3输入，第一个默认找最长的，第二个数和第一个等长，第三个数计算具体长度
                            if flags[0] == Flag::H {
                                // 默认打头的是H
                                let logic_block = LogicBlock::AOI21.if_rev(!target_wire.is_neg ^ is_out_inv);
                                let source_first_h_is_neg = !target_wire.is_neg ^ is_out_inv ^ custom_input_invs.contains(&0);
                                let source_first_h = manager.find(&AmbiguousWire::MayLenNotGivenOrSearchMax { 
                                    flag: Flag::H, 
                                    is_neg: source_first_h_is_neg, 
                                    index: target_wire.index, 
                                    may_len: custom_input_lens.get(&0).copied(), 
                                })?;
                                let source_first_h_len = source_first_h.len;

                                match (&flags[1], &flags[2]) {
                                    (Flag::P, Flag::G) | (Flag::Q, Flag::G) | (Flag::P, Flag::H) => {
                                        let source_pq_is_neg = !target_wire.is_neg ^ is_out_inv ^ custom_input_invs.contains(&1);
                                        let source_pq_index = target_wire.index - 1;
                                        let source_pg_len_shoice = if &flags[2] == &Flag::G && &flags[1] == &Flag::P {
                                            vec![source_first_h_len - 1, source_first_h_len]
                                        } else if &flags[2] == &Flag::H {
                                            vec![source_first_h_len]
                                        } else {
                                            vec![source_first_h_len-1]
                                        };
                                        let source_pq = manager.find(&AmbiguousWire::LenGivenByOrder  {
                                                flag: flags[1],
                                                index: source_pq_index,
                                                len_choice: source_pg_len_shoice,
                                                is_neg: source_pq_is_neg,
                                            
                                        })?;
                                        let source_end_g_is_neg = !target_wire.is_neg ^ is_out_inv ^ custom_input_invs.contains(&2);
                                        let source_end_g_index = target_wire.index - source_first_h_len;
                                        let source_end_g_len = target_wire.len - source_first_h_len;
                                        let source_end_g = manager.find(&AmbiguousWire::Precise (
                                            Wire {
                                                flag: flags[2],
                                                index: source_end_g_index,
                                                len: source_end_g_len,
                                                is_neg: source_end_g_is_neg,
                                            }
                                        ))?;
                                        Ok(Self::new_aoi21_like(logic_block, target_wire.clone(), 
                                            source_pq, source_end_g, source_first_h, 
                                            custom_input_invs.contains(&1), custom_input_invs.contains(&2), custom_input_invs.contains(&0), 
                                            *is_out_inv
                                        ))
                                    },
                                    _ => panic!("3 input gen G, index 1 and 2 flag can't be {:?}, {:?}", &flags[1],&flags[2])
                                }
                            } else if flags[0] == Flag::G {
                                // 新方法，从g合h
                                let logic_block = LogicBlock::AOI21.if_rev(!target_wire.is_neg ^ is_out_inv);
                                let source_first_g_is_neg = !target_wire.is_neg ^ is_out_inv ^ custom_input_invs.contains(&0);
                                let source_first_g = manager.find(&AmbiguousWire::MayLenNotGivenOrSearchMax { 
                                    flag: Flag::G, 
                                    is_neg: source_first_g_is_neg, 
                                    index: target_wire.index, 
                                    may_len: custom_input_lens.get(&0).copied(), 
                                })?;
                                let source_first_g_len = source_first_g.len;

                                match (&flags[1], &flags[2]) {
                                    (Flag::P, Flag::G) | (Flag::Q, Flag::G) | (Flag::P, Flag::H) => {
                                        let source_pq_is_neg = !target_wire.is_neg ^ is_out_inv ^ custom_input_invs.contains(&1);
                                        let source_pq_index = target_wire.index - 1;
                                        let source_pg_len_shoice = if &flags[2] == &Flag::G && &flags[1] == &Flag::P {
                                            vec![source_first_g_len - 1, source_first_g_len]
                                        } else if &flags[2] == &Flag::H {
                                            vec![source_first_g_len]
                                        } else {
                                            vec![source_first_g_len-1]
                                        };
                                        let source_pq = manager.find(&AmbiguousWire::LenGivenByOrder  {
                                                flag: flags[1],
                                                index: source_pq_index,
                                                len_choice: source_pg_len_shoice,
                                                is_neg: source_pq_is_neg,
                                            
                                        })?;
                                        let source_end_g_is_neg = !target_wire.is_neg ^ is_out_inv ^ custom_input_invs.contains(&2);
                                        let source_end_g_index = target_wire.index - source_first_g_len;
                                        let source_end_g_len = target_wire.len - source_first_g_len;
                                        let source_end_g = manager.find(&AmbiguousWire::Precise (
                                            Wire {
                                                flag: flags[2],
                                                index: source_end_g_index,
                                                len: source_end_g_len,
                                                is_neg: source_end_g_is_neg,
                                            }
                                        ))?;
                                        Ok(Self::new_aoi21_like(logic_block, target_wire.clone(), 
                                            source_pq, source_end_g, source_first_g, 
                                            custom_input_invs.contains(&1), custom_input_invs.contains(&2), custom_input_invs.contains(&0), 
                                            *is_out_inv
                                        ))
                                    },
                                    _ => panic!("3 input gen G, index 1 and 2 flag can't be {:?}, {:?}", &flags[1],&flags[2])
                                }
                            } else {
                                unimplemented!()
                            }
                        } else if flags.len() == 4 {
                            // 从AB直接出
                            assert_eq!(&flags[0], &Flag::A);
                            assert_eq!(&flags[1], &Flag::B);
                            assert_eq!(&flags[2], &Flag::P);
                            assert_eq!(&flags[3], &Flag::H);
                            let source_pq_index = target_wire.index;
                            let source_pq_len_shoice = vec![1];
                            // let source_pq_len_shoice = if &flags[3] == &Flag::G && &flags[2] == &Flag::P {
                            //     vec![0, 1]
                            // } else if &flags[3] == &Flag::H {
                            //     vec![1]
                            // } else {
                            //     vec![0]
                            // };
                            let source_pq = manager.find(&AmbiguousWire::LenGivenByOrder  {
                                flag: flags[2],
                                index: source_pq_index - 1,
                                len_choice: source_pq_len_shoice,
                                is_neg: false,
                            })?;
                            let source_end_g_index = target_wire.index - 1;
                            let source_end_g_len = target_wire.len - 1;
                            let source_end_g = manager.find(&AmbiguousWire::Precise (
                                Wire {
                                    flag: flags[3],
                                    index: source_end_g_index,
                                    len: source_end_g_len,
                                    is_neg: false,
                                }
                            ))?;
                            Ok(Self::new_from_vec(
                                LogicBlock::AOI22, 
                                vec![
                                    Wire::from_str(&format!("a{}",target_wire.index)), 
                                    Wire::from_str(&format!("b{}",target_wire.index)), 
                                    source_pq,
                                    source_end_g,
                                ], 
                                vec![target_wire.clone()], 
                            ))
                        } else {
                            unimplemented!()
                        }
                        
                        
                    }
                    Flag::P | Flag::Q => {
                        assert_eq!(flags.len(), 2);
                        let source_start_p_is_neg = !target_wire.is_neg ^ is_out_inv ^ custom_input_invs.contains(&0);
                        let source_end_p_is_neg = !target_wire.is_neg ^ is_out_inv ^ custom_input_invs.contains(&1);
                        let source_start_p = manager.find(&&AmbiguousWire::MayLenNotGivenOrSearchMax{
                            flag : flags[0],
                            is_neg : source_start_p_is_neg,
                            index: target_wire.index,
                            may_len: custom_input_lens.get(&0).copied(),
                        })?;
                        let source_start_p_len = source_start_p.len;
                        let source_end_p_index = target_wire.index - source_start_p_len;
                        let source_end_p_len = target_wire.len - source_start_p_len;
                        let source_end_p = manager.find(&AmbiguousWire::Precise(Wire {
                            flag: flags[1],
                            index: source_end_p_index,
                            len: source_end_p_len,
                            is_neg: source_end_p_is_neg,
                        }))?;
                        if source_start_p_is_neg == source_end_p_is_neg {
                            let logic_block = LogicBlock::ND2.if_rev(source_start_p_is_neg).if_add_out_inv(*is_out_inv);
                            Ok(Self::new_from_vec(
                                logic_block, 
                                vec![source_start_p, source_end_p],
                                vec![target_wire.clone()]
                            ))
                        } else {
                            Ok(Self::new_ind_inr_origin_is_and(target_wire.clone(), 
                                source_start_p, source_end_p, 
                                source_start_p_is_neg, source_end_p_is_neg, 
                                target_wire.is_neg,
                            ))
                        }
                    }
                    _ => {
                        unimplemented!()
                    }
                }
            }
        }
    }

    
}
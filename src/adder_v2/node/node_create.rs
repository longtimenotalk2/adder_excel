use core::panic;
use std::collections::BTreeMap;

use crate::adder_v2::{logic::{Logic, IO}, node::{Drive, FlagPChain, Node, NodeHint}, wire::{wire_list::WireList, Flag, FlagP, Wire, WireFloat, }, Id, Port};

pub struct FailParse {
    flag_p_chain : FlagPChain,
    found_wire : Vec<(Id, Wire)>,
    not_found_wire : Wire,
}

pub enum NodeCreateError {
    CanNotFindGivenWire(Wire),
    FailParse(Vec<FailParse>),
    NoChain(Wire),
    CanNotDirect(Wire),
}

impl FlagPChain {
    pub fn default_chains(flag : &Flag) -> Vec<Self> {
        let g = FlagP {flag: Flag::G, is_neg : false};
        let p = FlagP {flag: Flag::P, is_neg : false};
        let q = FlagP {flag: Flag::Q, is_neg : false};
        let h = FlagP {flag: Flag::H, is_neg : false};
        match flag {
            Flag::G => vec![
                FlagPChain(vec![g.clone(), p.clone(), g.clone()]),
                FlagPChain(vec![g.clone(), q.clone(), g.clone()]),
            ],
            Flag::P => vec![
                FlagPChain(vec![p.clone(), p.clone()]),
                FlagPChain(vec![q.clone(), p.clone()]),
            ],
            Flag::Q => vec![
                FlagPChain(vec![q.clone(), q.clone()]),
                FlagPChain(vec![p.clone(), q.clone()]),
            ],
            Flag::H => vec![
                FlagPChain(vec![h.clone(), p.clone(), h.clone()]),
            ],
            _ => vec![]
        }
    }
}

impl Node {
    pub fn create_from_hint(hint: &NodeHint, history_wires: &WireList) -> Result<Node, NodeCreateError> {

        let drive = hint.drive.clone();
        let id_next = history_wires.len() as Id;
        let index = hint.given_out_index;
        let len = hint.given_out_len;

        if hint.is_start_xnr_dout | hint.is_start_xor_dout | hint.is_start_xnr | hint.is_start_xor{
            let mut input = BTreeMap::new();
            input.insert(Port::new("A1"), history_wires.find(&Wire::from_str(&format!("a{index}")))?);
            input.insert(Port::new("A2"), history_wires.find(&Wire::from_str(&format!("b{index}")))?);

            if hint.is_start_xnr_dout | hint.is_start_xor_dout {
                let logic = if hint.is_start_xnr_dout { Logic::XNR2DOUT } else { Logic::XOR2DOUT };
                let z = (id_next, if hint.is_start_xnr_dout { Wire::from_str(&format!("nq{index}")) } else { Wire::from_str(&format!("q{index}")) });
                let o1 = (id_next + 1, if hint.is_start_xnr_dout { Wire::from_str(&format!("ng{index}")) } else { Wire::from_str(&format!("np{index}")) });
                return Ok(Node::new(
                    logic,
                    IO::<(Id, Wire)>::new(input, z, Some(o1)),
                    drive,
                ))
            } else {
                let logic = if hint.is_start_xnr { Logic::XNR2 } else { Logic::XOR2 };
                let z = (id_next, if hint.is_start_xnr { Wire::from_str(&format!("nq{index}")) } else { Wire::from_str(&format!("q{index}")) });
                return Ok(Node::new(
                    logic,
                    IO::<(Id, Wire)>::new(input, z, None),
                    drive,
                ))
            }
        } 

        let target_wire = Wire::from_logic_extend(
            hint.given_out_flag_p.clone().expect(&format!("hint {hint:?} must have flag extend")),
            hint.given_out_index,
            hint.given_out_len,
        );
        
        if hint.is_simple_inv {
            let input_wire = target_wire.to_rev();
            let found_wire = history_wires.find(&input_wire)?;
            let mut input = BTreeMap::new();
            input.insert(Port::new("I"), found_wire);
            return Ok(Node::new(
                Logic::INV,
                IO::<(Id, Wire)>::new(input, (id_next, target_wire), None),
                drive,
            ))
        } 

        if hint.is_start {
            let wire_float = target_wire.to_wire_float();
            let a1 = history_wires.find(&Wire::from_str(&format!("a{index}")))?;
            let b1 = history_wires.find(&Wire::from_str(&format!("b{index}")))?;
            let mut input = BTreeMap::new();
            input.insert(Port::new("A1"), a1);
            input.insert(Port::new("A2"), b1);

            if wire_float == WireFloat::from_str("ng") || wire_float == WireFloat::from_str("np") {
                return Ok(Node::new(
                    if wire_float == WireFloat::from_str("ng") {Logic::ND2} else {Logic::NR2},
                    IO::<(Id, Wire)>::new(input, (id_next, target_wire), None),
                    drive,
                ))
            }

            if wire_float == WireFloat::from_str("nh2") || wire_float == WireFloat::from_str("np2"){
                let sub_index = index - 1;
                let a0 = history_wires.find(&Wire::from_str(&format!("a{sub_index}")))?;
                let b0 = history_wires.find(&Wire::from_str(&format!("b{sub_index}")))?;
                input.insert(Port::new("B1"), a0);
                input.insert(Port::new("B2"), b0);
                return Ok(Node::new(
                    if wire_float == WireFloat::from_str("nh2") {Logic::AOI22} else {Logic::OAI22},
                    IO::<(Id, Wire)>::new(input, (id_next, target_wire), None),
                    drive,
                ))
            }
        }

        let is_out_addition_inv = hint.is_out_addition_inv;

        let extend_flag_chains = if let Some(chain) = &hint.given_flag_p_chain {
            vec![chain.clone()]
        } else {
            FlagPChain::default_chains(&target_wire.flag)
        };

        if extend_flag_chains.is_empty() {
            return Err(NodeCreateError::NoChain(target_wire));
        }

        let mut fail_parses: Vec<FailParse> = vec![];

        /*
        1. 假设输入都是正的，输出都是负的，生成基本的ND、IND/INR，AOI，IAOI等
        2. 考虑输入输出全取反，此时拿取镜像的logic
        3. 考虑输出强插一个INV，拿取强插后的logic
        */
        for extend_flag_chain in extend_flag_chains {
            let flag_chain = extend_flag_chain.0.iter().map(|ef| ef.clone().flag).collect::<Vec<_>>();
            let flags = flag_chain.iter().map(|f| f.to_str()).collect::<String>();
            match flags.as_str() {
                "PP" | "PQ" | "QP" | "QQ" | "PH" => todo!(),
                _ => panic!("can not parse flag chain {:?}", flag_chain),
            }
            // let mut two_input_swap = false;
            // let mut three_input_inv_index: Option<usize> = None;
            // let logic = match extend_flag_chain.0.len() {
            //     2 => {
            //         let a1_is_neg = extend_flag_chain.0[0].is_neg;
            //         let a2_is_neg = extend_flag_chain.0[1].is_neg;
            //         if a1_is_neg == a2_is_neg {
            //             Logic::ND2
            //         } else {
            //             let (logic, swap) = Logic::get_ind_inr_from_and(a1_is_neg, a2_is_neg, true);
            //             two_input_swap = swap;
            //             logic
            //         }
            //     },
            //     3 => {
            //         let b_is_neg = extend_flag_chain.0[0].is_neg;
            //         let a1_is_neg = extend_flag_chain.0[1].is_neg;
            //         let a2_is_neg = extend_flag_chain.0[2].is_neg;
            //         // 这仨最多只有1个是true
            //         let mut true_count = 0;
            //         if b_is_neg {true_count += 1};
            //         if a1_is_neg {true_count += 1};
            //         if a2_is_neg {true_count += 1};
            //         if true_count > 1 {
            //             panic!("wire {} has invalid flag chain {:?}, neg number should <= 1", target_wire.to_string(), extend_flag_chain);
            //         } else if true_count == 1 {
            //             if b_is_neg {
            //                 panic!("wire {} has invalid flag chain {:?}, b should not neg", target_wire.to_string(), extend_flag_chain)
            //             } else if a1_is_neg {
            //                 three_input_inv_index = Some(1);
            //             } else if a2_is_neg {
            //                 three_input_inv_index = Some(2);
            //             }
            //             Logic::IAOI21
            //         } else {
            //             Logic::AOI21
            //         }
            //     },
            //     4 => todo!(),
            //     _ => panic!("wire {} has invalid flag chain {:?}", target_wire.to_string(), extend_flag_chain),
            // };


        }

        Err(NodeCreateError::FailParse(fail_parses))

    }
}
use core::panic;
use std::collections::BTreeMap;

use colorful::{Color, Colorful};

use crate::adder_v2::{logic::{Logic, IO}, node::{pure_logic_layer::{FailParse, FlagIndexLen}, FlagPChain, Node, NodeHint}, wire::{wire_list::{self, WireList}, Flag, FlagP, Wire, WireFloat, }, Id, Port};

#[derive(Debug, Clone)]
pub enum NodeCreateError {
    CanNotFindGivenWire(Wire),
    FailParse(Wire, Vec<(FlagPChain, Vec<FailParse>)>),
    NoChain(Wire),
    CanNotDirect(Wire),
}

impl NodeCreateError {
    pub fn to_string(&self) -> String {
        match self {
            NodeCreateError::CanNotFindGivenWire(w) => format!("can not find given wire {}", w.to_string().color(Color::Red)),
            NodeCreateError::NoChain(w) => format!("syn wire {} need given chain", w.to_string().color(Color::Red)),
            NodeCreateError::CanNotDirect(w) => format!("syn wire {} can not direct from input", w.to_string().color(Color::Red)),
            NodeCreateError::FailParse(w, fail_parse) => {
                let mut s = String::new();
                for (chain, fail) in fail_parse {
                    s.push_str(&format!("syn wire {}: \n", w.to_string().color(Color::Yellow)));
                    s.push_str(&format!("chain {} fail parse: \n", chain.to_string()));
                    for f in fail {
                        s.push_str(&f.to_string());
                        s += "\n";
                    }
                }
                s
            }
        }
    }
}

impl FlagPChain {
    pub fn default_chains(flag : &Flag) -> Vec<Self> {
        let g = FlagP {flag: Flag::G, is_neg : false};
        let p = FlagP {flag: Flag::P, is_neg : false};
        let q = FlagP {flag: Flag::Q, is_neg : false};
        let h = FlagP {flag: Flag::H, is_neg : false};
        match flag {
            Flag::G => vec![
                FlagPChain(vec![g.clone(), q.clone(), g.clone()]),
                FlagPChain(vec![g.clone(), p.clone(), g.clone()]),
            ],
            Flag::P => vec![
                FlagPChain(vec![q.clone(), p.clone()]),
                FlagPChain(vec![p.clone(), p.clone()]),
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

        let id_next = history_wires.len() as Id;
        let index = hint.given_out_index;

        if hint.is_start_xnr_dout | hint.is_start_xor_dout | hint.is_start_xnr | hint.is_start_xor{
            let a1 = history_wires.find(&Wire::from_str(&format!("a{index}")))?;
            let a2 = history_wires.find(&Wire::from_str(&format!("b{index}")))?;

            if hint.is_start_xnr_dout | hint.is_start_xor_dout {
                let logic = if hint.is_start_xnr_dout { Logic::XNR2DOUT } else { Logic::XOR2DOUT };
                let z = (id_next + 1, if hint.is_start_xnr_dout { Wire::from_str(&format!("nq{index}")) } else { Wire::from_str(&format!("q{index}")) });
                let o1 = (id_next, if hint.is_start_xnr_dout { Wire::from_str(&format!("ng{index}")) } else { Wire::from_str(&format!("np{index}")) });
                return Ok(Node::create_by_ordered_wires(
                    logic,
                    vec![a1, a2, o1, z],
                ))
            } else {
                let logic = if hint.is_start_xnr { Logic::XNR2 } else { Logic::XOR2 };
                let z = (id_next, if hint.is_start_xnr { Wire::from_str(&format!("nq{index}")) } else { Wire::from_str(&format!("q{index}")) });
                return Ok(Node::create_by_ordered_wires(
                    logic,
                    vec![a1, a2, z],
                ))
            }
        } 

        let target_wire = Wire::from_flag_p(
            hint.given_out_flag_p.clone().expect(&format!("hint {hint:?} must have flag extend")),
            hint.given_out_index,
            hint.given_out_len,
        );

        // dbg!(&target_wire);
        
        if hint.is_simple_inv {
            let input_wire = target_wire.to_rev();
            let found_wire = history_wires.find(&input_wire)?;;
            return Ok(Node::create_by_ordered_wires(
                Logic::INV,
                vec![found_wire, (id_next, target_wire)],
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
                ))
            }
        }

        let is_out_addition_inv = hint.is_out_addition_inv;

        let flagp_chains = if let Some(chain) = &hint.given_flag_p_chain {
            vec![chain.clone()]
        } else {
            FlagPChain::default_chains(&target_wire.flag)
        };

        if flagp_chains.is_empty() {
            return Err(NodeCreateError::NoChain(target_wire));
        }

        // polar layer，搞清楚每个输入的极性
        let mut input_is_neg = !target_wire.is_neg;  // 通常，输入与输出相反
        input_is_neg = input_is_neg ^ is_out_addition_inv;  // 如果输出要加额外inv，那么反转结果

        let mut fail_parse_conditions: Vec<(FlagPChain, Vec<FailParse>)> = vec![];

        for flagp_chain in &flagp_chains {
            // 输入的flagp_chain，默认输入都是正
            // true_flagp_chain，要处理默认输入为反的情况
            let true_flagp_chain = FlagPChain(flagp_chain.0.iter().map(|fp| if input_is_neg {fp.to_rev()} else {fp.clone()}).collect::<Vec<FlagP>>());
            // 特殊配置特殊处理
            if true_flagp_chain == FlagPChain(vec![
                FlagP::new(Flag::A, false), 
                FlagP::new(Flag::B, false),
                FlagP::new(Flag::Q, false),
                FlagP::new(Flag::G, false),
            ]) {
                let a1 = history_wires.find(&Wire::from_str(&format!("a{index}")))?;
                let a2 = history_wires.find(&Wire::from_str(&format!("b{index}")))?;
                let b1 = history_wires.find(&Wire::from_str(&format!("q{index}")))?;
                let b2 = history_wires.find(&Wire::new(Flag::G, false, index-1, hint.given_out_len-1))?;
                return Ok(Node::create_by_ordered_wires(
                    Logic::AOI22,
                    vec![a1, a2, b1, b2, (id_next, target_wire)],
                ))
            }
            if true_flagp_chain == FlagPChain(vec![
                FlagP::new(Flag::A, false), 
                FlagP::new(Flag::B, false),
                FlagP::new(Flag::P, false),
                FlagP::new(Flag::H, false),
            ]) {
                let a1 = history_wires.find(&Wire::from_str(&format!("a{index}")))?;
                let a2 = history_wires.find(&Wire::from_str(&format!("b{index}")))?;
                let b1 = history_wires.find(&Wire::from_str(&format!("p{}", index-1)))?;
                let b2 = history_wires.find(&Wire::new(Flag::H, false, index-1, hint.given_out_len-1))?;
                return Ok(Node::create_by_ordered_wires(
                    Logic::AOI22,
                    vec![a1, a2, b1, b2, (id_next, target_wire)],
                ))
            }
            if true_flagp_chain == FlagPChain(vec![
                FlagP::new(Flag::A, false), 
                FlagP::new(Flag::B, false),
                FlagP::new(Flag::G, false),
            ]) {
                let a1 = history_wires.find(&Wire::from_str(&format!("a{index}")))?;
                let a2 = history_wires.find(&Wire::from_str(&format!("b{index}")))?;
                let b  = history_wires.find(&Wire::new(Flag::G, false, index-1, hint.given_out_len-1))?;
                return Ok(Node::create_by_ordered_wires(
                    Logic::AOI21,
                    vec![b, a1, a2, (id_next, target_wire)],
                ))
            }

            // 其余一般情况
            let solve_result = history_wires.solve_pure_logic_layer(
                &true_flagp_chain, 
                &target_wire,
                hint.is_use_mirror,
            );
            match solve_result {
                Ok(mut node) => {
                    // polar layer，处理各种极性问题
                    // 处理初始极性颠倒
                    if input_is_neg {
                        node.logic = node.logic.mirror();
                    }
                    // 处理部分输入INV
                    let mut input_inv_count = 0;
                    let mut position = None;
                    for (i, fp) in flagp_chain.0.iter().enumerate() {
                        if fp.is_neg {
                            input_inv_count += 1;
                            position = Some(i);
                        }
                    }
                    if input_inv_count > 1 {
                        panic!("input_inv_count > 1, in {:?}", flagp_chain);
                    } else if input_inv_count == 1 {
                        let neg_position = position.unwrap();
                        node = node.impl_input_inv(neg_position)
                    }
                    // 处理输出部分INV
                    if is_out_addition_inv {
                        node = node.impl_output_inv();
                    }
                    return Ok(node)
                }
                Err(err) => {
                    fail_parse_conditions.push((flagp_chain.clone(), err));
                }
            }
        }

        Err(NodeCreateError::FailParse(target_wire, fail_parse_conditions))

    }
}
use std::collections::BTreeMap;

use crate::adder_v2::{logic::{Logic, IO}, node::{Drive, Node, NodeHint}, wire::Wire, Port};

impl Node {
    pub fn create_from_hint(hint: &NodeHint, history_wires: &[Wire]) -> Node {

        let drive = hint.drive.clone();

        if hint.is_start_xnr_dout | hint.is_start_xor_dout | hint.is_start_xnr | hint.is_start_xor{
            let index = hint.given_out_index;
            let mut input = BTreeMap::new();
            input.insert(Port::new("A1"), Wire::from_str(&format!("a{index}")));
            input.insert(Port::new("A2"), Wire::from_str(&format!("b{index}")));

            if hint.is_start_xnr_dout | hint.is_start_xor_dout {
                let logic = if hint.is_start_xnr_dout { Logic::XNR2DOUT } else { Logic::XOR2DOUT };
                let o1 = if hint.is_start_xnr_dout { Wire::from_str(&format!("ng{index}")) } else { Wire::from_str(&format!("np{index}")) };
                let z = if hint.is_start_xnr_dout { Wire::from_str(&format!("nq{index}")) } else { Wire::from_str(&format!("q{index}")) };
                return Node::new(
                    logic,
                    IO::new(input, z, Some(o1)),
                    drive,
                )
            } else {
                let logic = if hint.is_start_xnr { Logic::XNR2 } else { Logic::XOR2 };
                let z = if hint.is_start_xnr { Wire::from_str(&format!("nq{index}")) } else { Wire::from_str(&format!("q{index}")) };
                return Node::new(
                    logic,
                    IO::new(input, z, None),
                    drive,
                )
            }
        } 

        let target_wire = Wire::from_logic_extend(
            hint.given_out_flag_extend.clone().expect(&format!("hint {hint:?} must have flag extend")),
            hint.given_out_index,
            hint.given_out_len,
        );
        
        if hint.is_simple_inv {
            // let target_wire = 
        } 

        unimplemented!()

    }
}
use std::collections::BTreeMap;

use crate::adder_v2::{logic::{Logic, IO}, node::{Drive, Node, NodeHint}, wire::{wire_list::WireList, Wire}, Id, Port};

pub enum NodeCreateError {
    CanNotFindGivenWire(Wire),
}

impl Node {
    pub fn create_from_hint(hint: &NodeHint, history_wires: &WireList) -> Result<Node, NodeCreateError> {

        let drive = hint.drive.clone();
        let id_next = history_wires.len() as Id;

        if hint.is_start_xnr_dout | hint.is_start_xor_dout | hint.is_start_xnr | hint.is_start_xor{
            let index = hint.given_out_index;
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
            hint.given_out_flag_extend.clone().expect(&format!("hint {hint:?} must have flag extend")),
            hint.given_out_index,
            hint.given_out_len,
        );
        
        if hint.is_simple_inv {
            let input_wire = target_wire.to_rev();
        } 

        unimplemented!()

    }
}
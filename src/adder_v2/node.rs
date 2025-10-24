pub mod node_create;
pub mod pure_logic_layer;

use std::collections::BTreeMap;

use crate::adder_v2::{logic::{Logic, IO}, wire::{Flag, FlagP, Wire}, Id, Port};

#[derive(Debug, Clone)]
pub enum Drive {
    D1,
    D2,
}

#[derive(Debug, Clone)]
pub struct Node {
    logic : Logic,
    io : IO<(Id, Wire)>,
}

#[derive(Debug, Clone)]
pub struct FlagPChain(pub Vec<FlagP>);


#[derive(Debug, Clone)]
pub struct NodeHint {
    is_simple_inv : bool,
    is_start : bool,
    is_start_xnr_dout : bool,
    is_start_xor_dout : bool,
    is_start_xnr : bool,
    is_start_xor : bool,
    drive : Drive,
    given_out_flag_p : Option<FlagP>,
    given_out_index : usize,
    given_out_len : usize,
    given_flag_p_chain : Option<FlagPChain>, 
    is_out_addition_inv : bool,
}


impl Node {
    pub fn new(logic : Logic, io : IO<(Id, Wire)>) -> Self {
        Self {
            logic,
            io,
        }
    }

    pub fn create_by_ordered_wires(logic : Logic, wires : Vec<(Id, Wire)>) -> Self {
        // AOA按照从外（C）到里（A1、A2）的顺序
        // 输出O1在Z前面
        let mut inputs: BTreeMap<Port, (u32, Wire)> = BTreeMap::new();

        let input_len = logic.input_port_ordered().len();

        for (i, port) in logic.input_port_ordered().iter().enumerate() {
            inputs.insert(port.clone(), wires[i].clone());
        }

        let io = match logic {
            Logic::XNR2DOUT | Logic::XOR2DOUT => {
                IO::new(inputs, wires[3].clone(), Some(wires[2].clone()))
            }
            _ => {
                IO::new(inputs, wires[input_len].clone(), None)
            }
        };
        Self::new(logic, io)
    }

    pub fn wire_swap(&mut self, port1 : usize, pos2 : usize) {
        
    }

    // pub fn impl_input_inv(&mut self, pos : usize) {
    //     match logic {
    //         Logic::ND2 | Logic::NR2 {
    //             let new_logic = match logic {
    //                 Logic::ND2 => Logic::IND2,
    //                 Logic::NR2 => Logic::INR2,
    //             };

    //         },
    //         _  => panic!("{logic:?} can not impl input inv")
    //     }
    // }
}
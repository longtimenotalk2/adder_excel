pub mod node_create;
pub mod pure_logic_layer;

use std::collections::BTreeMap;

use crate::adder_v2::{logic::{Logic, IO}, wire::{Flag, FlagP, Wire}, Id, Port};



#[derive(Debug, Clone)]
pub struct Node {
    logic : Logic,
    io : IO<(Id, Wire)>,
}

#[derive(Debug, Clone)]
pub struct FlagPChain(pub Vec<FlagP>);

impl FlagPChain {
    pub fn to_string(&self) -> String {
        self.0.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(",")
    }
}


#[derive(Debug, Clone, Default)]
pub struct NodeHint {
    pub is_simple_inv : bool,
    pub is_start : bool,
    pub is_start_xnr_dout : bool,
    pub is_start_xor_dout : bool,
    pub is_start_xnr : bool,
    pub is_start_xor : bool,
    pub given_out_flag_p : Option<FlagP>,
    pub given_out_index : usize,
    pub given_out_len : usize,
    pub given_flag_p_chain : Option<FlagPChain>, 
    pub is_out_addition_inv : bool,
}


impl Node {
    pub fn new(logic : Logic, io : IO<(Id, Wire)>) -> Self {
        Self {
            logic,
            io,
        }
    }

    pub fn get_ordered_input_wires(&self) -> Vec<(Id, Wire)> {
        self.logic.input_port_ordered().iter().map(|port| self.io.input.get(port).cloned().unwrap()).collect()
    }

    pub fn get_ordered_output_wires(&self) -> Vec<(Id, Wire)> {
        let mut ret = vec![];
        if let Some(o1) = self.io.output_o1.clone() {
            ret.push(o1);
        }
        ret.push(self.io.output_z.clone());
        ret
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
                IO::new(inputs, wires[input_len-1].clone(), None)
            }
        };
        Self::new(logic, io)
    }

    pub fn impl_input_inv(&self, pos : usize) -> Self {
        match self.logic {
            Logic::ND2 | Logic::NR2 => {
                let new_logic = match self.logic {
                    Logic::ND2 => Logic::IND2,
                    Logic::NR2 => Logic::INR2,
                    _ => unimplemented!()
                };
                let wires: Vec<(u32, Wire)> = match pos {
                    0 => self.get_ordered_input_wires(),
                    1 => {
                        self.get_ordered_input_wires().into_iter().rev().collect()
                    }
                    _ => unimplemented!()
                };
                Self::create_by_ordered_wires(new_logic, wires)
            },
            Logic::AOI21 | Logic::OAI21 => {
                let new_logic = match self.logic {
                    Logic::AOI21 => Logic::IAOI21,
                    Logic::OAI21 => Logic::IOAI21,
                    _ => unimplemented!()
                };
                let wires: Vec<(u32, Wire)> = match pos {
                    0 => panic!("IAOI B can not add inv"),
                    1 => {
                        let wires = self.get_ordered_input_wires();
                        let mut new_wires = vec![];
                        new_wires.push(wires[0].clone());
                        new_wires.push(wires[2].clone());
                        new_wires.push(wires[1].clone());
                        new_wires
                    }
                    2 => self.get_ordered_input_wires(),
                    _ => unimplemented!()
                };
                Self::create_by_ordered_wires(new_logic, wires)
            }
            _  => panic!("{:?} can not impl input inv", self.logic)
        }
    }

    pub fn impl_output_inv(&self) -> Self {
        match self.logic {
            Logic::ND2 | Logic::NR2 | Logic::AOI21 | Logic::OAI21 | Logic::AOI22 | Logic::OAI22 | Logic::AOAI211 | Logic::OAOI211  => {
                let new_logic = match self.logic {
                    Logic::ND2 => Logic::AN2,
                    Logic::NR2 => Logic::OR2,
                    Logic::AOI21 => Logic::OA21,
                    Logic::OAI21 => Logic::OA21,
                    Logic::AOAI211 => Logic::AOA211,
                    Logic::OAOI211 => Logic::OAO211,
                    _ => unimplemented!()
                };
                let wires: Vec<(u32, Wire)> = self.get_ordered_input_wires();
                Self::create_by_ordered_wires(new_logic, wires)
            },
            Logic::IND2 | Logic::INR2 => {
                // cell互换+输入互换
                let wires = self.get_ordered_input_wires().into_iter().rev().collect::<Vec<_>>();
                let new_logic = match self.logic {
                    Logic::IND2 => Logic::INR2,
                    Logic::INR2 => Logic::IND2,
                    _ => unimplemented!()
                };
                Self::create_by_ordered_wires(new_logic, wires)
            },
            _ => panic!("{:?} can not impl output inv", self.logic)
        }
    }
}
pub mod node_create;
pub mod pure_logic_layer;

use std::collections::BTreeMap;

use crate::adder_v2::{logic::{Logic, IO}, wire::{wire_list::WireList, Flag, FlagP, FlagPM, Wire}, Id, Port};



#[derive(Debug, Clone)]
pub struct Node {
    pub logic : Logic,
    pub io : IO<(Id, Wire)>,
}

impl Node {
    pub fn to_string(&self) -> String {
        let inputs = self.get_ordered_input_wires().iter().map(|(_, w)| w.to_string()).collect::<Vec<_>>().join(", ");
        let outputs = self.get_ordered_output_wires().iter().map(|(_, w)| w.to_string()).collect::<Vec<_>>().join(", ");
        format!("{}({inputs}) -> {outputs}", self.logic.to_string())
    }

    pub fn to_inst_name_with_layer(&self, layer : i32) -> String {
        let mut name = format!("U{}", layer);
        if let Some((_, o1_wire)) = self.io.output_o1.as_ref() {
            name.push_str("_");
            name.push_str(&o1_wire.to_string());
        }
        name.push_str("_");
        name.push_str(&(self.io.output_z.1.to_string()));
        name
    }

    pub fn to_port_vs_wire_name(&self) -> BTreeMap<Port, String> {
        let mut map = BTreeMap::new();
        for (port, (_, wire)) in self.io.input.iter() {
            map.insert(port.clone(), wire.to_string());
        }
        if let Some((_, o1_wire)) = self.io.output_o1.as_ref() {
            map.insert(Port::new("O1"), o1_wire.to_string());
        }
        map.insert(self.logic.z_port(),  self.io.output_z.1.to_string());

        map
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub given_out_flag_pm : Option<FlagPM>,
    pub given_out_index : usize,
    pub given_out_len : usize,
    pub given_flag_p_chain : Option<FlagPChain>, 
    pub is_out_addition_inv : bool,
    pub give_final_c : bool, // 其输出的c会强制给到末尾异或
    pub give_final_q : bool, // 其输出的q会强制给到末尾异或
    pub is_use_mirror : bool,
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

    pub fn get_ordered_all_wires(&self) -> Vec<(Id, Wire)> {
        let mut ret = self.get_ordered_input_wires();
        ret.extend(self.get_ordered_output_wires());
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
                IO::new(inputs, wires[input_len].clone(), None)
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
                let mut wires: Vec<(u32, Wire)> = match pos {
                    0 => self.get_ordered_input_wires(),
                    1 => {
                        self.get_ordered_input_wires().into_iter().rev().collect()
                    }
                    _ => unimplemented!()
                };
                wires.append(&mut self.get_ordered_output_wires());
                Self::create_by_ordered_wires(new_logic, wires)
            },
            
            Logic::AOI21 | Logic::OAI21 => {
                let new_logic = match self.logic {
                    Logic::AOI21 => Logic::IAOI21,
                    Logic::OAI21 => Logic::IOAI21,
                    _ => unimplemented!()
                };
                let mut wires: Vec<(u32, Wire)> = match pos {
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
                wires.append(&mut self.get_ordered_output_wires());
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
                let wires: Vec<(u32, Wire)> = self.get_ordered_all_wires();
                Self::create_by_ordered_wires(new_logic, wires)
            },
            Logic::IND2 | Logic::INR2 => {
                // cell互换+输入互换
                let mut wires = self.get_ordered_input_wires().into_iter().rev().collect::<Vec<_>>();
                wires.append(&mut self.get_ordered_output_wires());
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
    
    pub fn calc_with_list(&self, value_tabel : &mut BTreeMap<(Id, Wire), bool>) {
        let mut values = self.io.input.iter().map(|(port, wire)| (port.clone(), *value_tabel.get(wire).unwrap())).collect::<BTreeMap<Port, bool>>();
        let (z_value, o1_value) = self.logic.calc(&values);
        value_tabel.insert(self.io.output_z.clone(), z_value);
        if let Some(o1_value) = o1_value {
            value_tabel.insert(self.io.output_o1.clone().unwrap(), o1_value);
        }
    }
}
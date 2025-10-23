use std::collections::{BTreeMap, BTreeSet};

use crate::{cell_parse::{ProcessAndProject, RealCell}, std::{adder::Adder, logic_block::Port, wire::{Flag, Wire}}};

impl Wire {
    fn is_not_a_b_s(&self) -> bool {
        match self.flag {
            Flag::S | Flag::A | Flag::B => false,
            _ => true
        }
    }

    fn to_string_netlist(&self) -> String {
        match self.flag {
            Flag::S => format!("s[{}]", self.index),
            Flag::A => format!("a[{}]", self.index),
            Flag::B => format!("b[{}]", self.index),
            _ => self.to_string()
        }
    }
}

impl Adder {
    pub fn to_netlist(&self, module_name : &str, process : ProcessAndProject) -> String {
        let bit_up = self.bits - 1;
        let mut ret = format!("module {module_name} (
	a, 
	b, 
	s);
   input [{bit_up}:0] a;
   input [{bit_up}:0] b;
   output [{bit_up}:0] s;

   // Internal wires\n");

        struct InstLine {
            inst_name : String,
            cell_name : String,
            port_and_wire : BTreeMap<Port, Wire>
        }

        let mut wires = BTreeSet::new();
        let mut inst_line: Vec<InstLine> = vec![];

        for cell in self.cells.iter() {
            let input = &cell.logic_block_map.inputs;
            let output = &cell.logic_block_map.outputs;

            let mut all_port_and_wire = BTreeMap::new(); 

            for (port, wire) in input.iter() {
                all_port_and_wire.insert(port.clone(), wire.clone());
            }

            for (port, wire) in output.iter() {
                all_port_and_wire.insert(port.clone(), wire.clone());
            }

            for (_, wire) in &all_port_and_wire {
                if wire.is_not_a_b_s() {
                    wires.insert(wire.clone());
                }
            }

            let real_cell = RealCell::parse(process, &cell.to_abstract_cell());

            inst_line.push(InstLine {
                inst_name: cell.inst_name(),
                cell_name: real_cell.name,
                port_and_wire: all_port_and_wire,
            })
        }

        for wire in wires.iter() {
            ret += &format!("   wire {};\n", wire.to_string_netlist());
        }

        ret += "\n";

        for inst in inst_line.iter() {
            let mut txt = String::new();
            txt += &format!("   {}", inst.cell_name);
            txt += &format!(" {}(", inst.inst_name);
            for (i, (port, wire)) in inst.port_and_wire.iter().rev().enumerate() {
                if i != 0 {
                    txt += ",\n\t";
                }
                txt += &format!(".{}({})", port.0, wire.to_string_netlist());
            }
            txt += ");\n";
            ret += &txt;
        }

        ret += "endmodule";

        ret
    }
}
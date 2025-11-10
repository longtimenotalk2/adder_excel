use std::collections::{BTreeMap, BTreeSet};

use crate::adder_v2::{adder::Adder, cell_parse::Process, wire::{Flag, Wire}, Id, Port};

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
    pub fn to_netlist(&self, module_name : &str, process : Process, cell_name_replace_map : BTreeMap<Id, String>) -> String {
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
            port_and_wire : BTreeMap<Port, (Id, Wire)>
        }

        let mut wires: BTreeSet<(u32, Wire)> = BTreeSet::new();
        let mut inst_line: Vec<InstLine> = vec![];

        let layers = self.scan_layer_end_same();

        for (cell_id, cell) in self.cells.iter() {

            let layer = layers.get(*cell_id as usize).unwrap();

            let all_port_and_wire = cell.node.to_port_vs_wire();

            for (_, wire) in &all_port_and_wire {
                if wire.1.is_not_a_b_s() {
                    wires.insert(wire.clone());
                }
            }

            inst_line.push(InstLine {
                inst_name: cell.node.to_inst_name_with_layer(*layer),
                cell_name: {
                    if let Some(replaced_cell_name) = cell_name_replace_map.get(cell_id) {
                        replaced_cell_name.clone()
                    } else {
                        cell.to_cell_body().parse(process).0.0
                    }
                },
                port_and_wire: all_port_and_wire,
            })
        }

        for wire in wires.iter() {
            ret += &format!("   wire {};\n", wire.1.to_string_netlist());
        }

        ret += "\n";

        for inst in inst_line.iter() {
            let mut txt = String::new();
            txt += &format!("   {}", inst.cell_name);
            txt += &format!(" {} (", inst.inst_name);
            for (i, (port, wire)) in inst.port_and_wire.iter().rev().enumerate() {
                if i != 0 {
                    txt += ",\n\t";
                }
                txt += &format!(".{}({})", port.0, wire.1.to_string_netlist());
            }
            txt += ");\n";
            ret += &txt;
        }

        ret += "endmodule";

        ret
    }
}
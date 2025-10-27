use std::collections::BTreeMap;

use colorful::{Color, Colorful};

use crate::adder_v2::{adder::Adder, wire::{self, wire_list::WireList}, Id};

impl Adder {
    pub fn check_id_all_match(&self) {
        print!(">>> check_id_all_match ...  ");
        // wires id all match
        for (i, (id, wire)) in self.wires.iter().enumerate() {
            if i as Id != *id {
                let mut txt = String::new();
                txt += "wire list :\n"; 
                for wire in &self.wires {
                    txt += &format!("> {:03} : {}\n", wire.0, wire.1.to_string());
                }
                println!("{}", txt);
                panic!("wire {} id = {id}, but in list index {i}", wire.to_string());
            }
        }
        // cell id all match
        for (i, (id, cell)) in self.cells.iter().enumerate() {
            if i as Id != *id {
                let mut txt = String::new();
                txt += "wire list :\n"; 
                for wire in &self.wires {
                    txt += &format!("> {:03} : {}\n", wire.0, wire.1.to_string());
                }
                println!("{}", txt);
                panic!("cell {} id = {id}, but in list index {i}", cell.to_string());
            }
        }
        // check all cell's wire is in wire list
        for (_, cell) in &self.cells {
            let wires = &cell.node.get_ordered_all_wires();
            for (wire_id, wire) in wires {
                if let Some((_, get_wire)) = self.wires.get(*wire_id as usize) {
                    if get_wire != wire {
                        panic!("cell {} wire {} id {wire_id} not match", cell.to_string(), wire.to_string());
                    }
                } else {
                    panic!("cell {} wire {} id {wire_id} not in wire list", cell.to_string(), wire.to_string());
                }
            }

        }
        println!("{}", "pass !".to_string().color(Color::Green));
    }
}
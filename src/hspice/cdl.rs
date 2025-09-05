use std::collections::BTreeSet;

use crate::{cell_parse::{ProcessAndProject, RealCell}, hspice::*, std::adder::Adder};

impl Adder {

    fn adder_pins(&self, process : ProcessAndProject) -> Vec<String> {
        let mut ports = vec![];
        for i in 0..self.bits {
            ports.push(format!("A{i}"));
        }
        for i in 0..self.bits {
            ports.push(format!("B{i}"));
        }
        for i in 0..self.bits {
            ports.push(format!("S{i}"));
        }
        let mut pg_port = BTreeSet::from(["VBB".to_string(), "VDD".to_string(), "VPP".to_string(), "VSS".to_string()]);
        for cell in self.all_custom_abstract_cells() {
            let real_cell = RealCell::parse(process, &cell);
            for port in real_cell.addition_pg_port {
                pg_port.insert(port.0.to_string());
            }
        }
        ports.append(&mut pg_port.into_iter().collect::<Vec<_>>());
        
        ports
    }

    fn adder_pins_two_index(&self, process : ProcessAndProject) -> Vec<String> {
        let mut ports = vec![];
        for i in 0..self.bits {
            ports.push(format!("A{i:02}"));
        }
        for i in 0..self.bits {
            ports.push(format!("B{i:02}"));
        }
        for i in 0..self.bits {
            ports.push(format!("S{i:02}"));
        }
        let mut pg_port = BTreeSet::from(["VBB".to_string(), "VDD".to_string(), "VPP".to_string(), "VSS".to_string()]);
        for cell in self.all_custom_abstract_cells() {
            let real_cell = RealCell::parse(process, &cell);
            for port in real_cell.addition_pg_port {
                pg_port.insert(port.0.to_string());
            }
        }
        ports.append(&mut pg_port.into_iter().collect::<Vec<_>>());
        
        ports
    }

    fn adder_pins_two_index_all_vdd_split(&self, process : ProcessAndProject) -> Vec<String> {
        let mut ports = vec![];
        for i in 0..self.bits {
            ports.push(format!("A{i:02}"));
        }
        for i in 0..self.bits {
            ports.push(format!("B{i:02}"));
        }
        for i in 0..self.bits {
            ports.push(format!("S{i:02}"));
        }
        let mut pg_port = BTreeSet::from(["VBB".to_string(), "VDD".to_string(), "VPP".to_string(), "VSS".to_string()]);
        for cell in self.all_custom_abstract_cells() {
            let real_cell = RealCell::parse(process, &cell);
            for port in real_cell.addition_pg_port {
                pg_port.insert(port.0.to_string());
            }
        }
        let mut pg_port_new = vec![];
        for port in pg_port {
            if port == "VDD" {
                for cell in &self.cells {
                    pg_port_new.push(format!("VDD_{}", cell.inst_name()));
                }
            } else {
                pg_port_new.push(port);
            }
        }
        ports.append(&mut pg_port_new.into_iter().collect::<Vec<_>>());
        
        ports
    }

    fn adder_pins_with(&self, process : ProcessAndProject, a_s : &[String], b_s : &[String], s_s : &[String]) -> Vec<String> {
        let mut ports = vec![];
        for a in a_s {
            ports.push(a.to_string());
        }
        for b in b_s {
            ports.push(b.to_string());
        }
        for s in s_s {
            ports.push(s.to_string());
        }
        let mut pg_port = BTreeSet::from(["VBB".to_string(), "VDD".to_string(), "VPP".to_string(), "VSS".to_string()]);
        for cell in self.all_custom_abstract_cells() {
            let real_cell = RealCell::parse(process, &cell);
            for port in real_cell.addition_pg_port {
                pg_port.insert(port.0.to_string());
            }
        }
        ports.append(&mut pg_port.into_iter().collect::<Vec<_>>());
        
        ports
    }

    pub fn adder_call(&self, process : ProcessAndProject, inst_name : &str) -> String {
        let mut lens = vec![];
        let mut remains = self.bits;
        while remains >= 8 {
            lens.push(8);
            remains -= 8;
        }
        lens.push(remains);
        let nets = self.adder_pins_two_index(process);
        lens = [lens.clone(), lens.clone(), lens.clone(), vec![nets.len() - 3 * self.bits]].concat();
        line_cell_given_lens("ADDER", &self.adder_pins_two_index(process), inst_name, &lens)
    }

    pub fn adder_call_all_vdd_split(&self, process : ProcessAndProject, inst_name : &str) -> String {
        let mut lens = vec![];
        let mut remains = self.bits;
        while remains >= 8 {
            lens.push(8);
            remains -= 8;
        }
        lens.push(remains);
        let nets = self.adder_pins_two_index_all_vdd_split(process);
        lens = [lens.clone(), lens.clone(), lens.clone(), vec![nets.len() - 3 * self.bits]].concat();
        line_cell_given_lens("ADDER", &self.adder_pins_two_index_all_vdd_split(process), inst_name, &lens)
    }

    pub fn adder_call_with(&self, process : ProcessAndProject, inst_name : &str, a_s : &[String], b_s : &[String], s_s : &[String]) -> String {
        let mut lens = vec![];
        let mut remains = self.bits;
        while remains >= 8 {
            lens.push(8);
            remains -= 8;
        }
        lens.push(remains);
        let nets = self.adder_pins_with(process, a_s, b_s, s_s);
        lens = [lens.clone(), lens.clone(), lens.clone(), vec![nets.len() - 3 * self.bits]].concat();
        line_cell_given_lens("ADDER_2ND", &self.adder_pins_with(process, a_s, b_s, s_s), inst_name, &lens)
    }
    


    pub fn to_cdl_std(&self, process : ProcessAndProject, name : &str) -> String {
        let mut txt = String::new();

        // inc cells
        for abstract_cells in &self.all_abstract_cells() {
            txt += &RealCell::parse(process, abstract_cells).line_inc();
        }

        txt += "\n";

        // subckt
        let ports = self.adder_pins(process);
        txt += &line_subckt(name, &ports);
        
        // cells
        for cell_info in self.cells.iter() {
            let inst_name = cell_info.inst_name();
            let map = &cell_info.logic_block_map;
            let abstract_cell = &cell_info.to_abstract_cell();
            let real_cell = RealCell::parse(process, abstract_cell);
            txt += &real_cell.line_cell(&inst_name, map);
        }

        // end
        txt += &line_end_subckt();

        txt
    }

    pub fn to_cdl_all_vdd_split(&self, process : ProcessAndProject, name : &str) -> String {
        let mut txt = String::new();

        // inc cells
        for abstract_cells in &self.all_abstract_cells() {
            txt += &RealCell::parse(process, abstract_cells).line_inc();
        }

        txt += "\n";

        // subckt
        let ports_std = self.adder_pins(process);
        let mut ports = vec![];
        for port in ports_std {
            if port == "VDD" {
                for cell in &self.cells {
                    ports.push(format!("VDD_{}", cell.inst_name()));
                }
            } else {
                ports.push(port);
            }
        }
        txt += &line_subckt(name, &ports);
        
        // cells
        for cell_info in self.cells.iter() {
            let inst_name = cell_info.inst_name();
            let map = &cell_info.logic_block_map;
            let abstract_cell = &cell_info.to_abstract_cell();
            let real_cell = RealCell::parse(process, abstract_cell);
            txt += &real_cell.line_cell_vdd_split(&inst_name, map);
        }

        // end
        txt += &line_end_subckt();

        txt
    }
}
use crate::{cell_parse::{ProcessAndProject, RealCell}, hspice::line_subckt, std::adder::Adder};

impl Adder {
    pub fn to_cdl_std(&self, process : ProcessAndProject, name : &str) -> String {
        let mut txt = String::new();

        // inc cells
        for abstract_cells in &self.all_abstract_cells() {
            txt += &RealCell::parse(process, abstract_cells).line_inc();
        }

        txt += "\n";

        // subckt
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
        ports.append(&mut vec!["VBB".to_string(), "VDD".to_string(), "VPP".to_string(), "VSS".to_string()]);
        txt += &line_subckt(name, &ports);
        
        // cells
        


        txt
    }
}
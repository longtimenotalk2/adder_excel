use crate::{adder_v2::{adder::Adder, cell_parse::Process}, hspice::{line_cap, line_end_subckt, line_inc, line_subckt}};

impl Adder {
    pub fn to_cdl(&self, name : &str, process : Process) -> String {
        let mut txt = String::new();

        // include
        for cell_body in self.cell_body_set() {
            txt += &line_inc(&cell_body.spf_path(process))
        }
        txt += "\n";

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
        for pg_port in self.pg_ports(process) {
            ports.push(pg_port.0);
        }

        txt += &line_subckt(name, &ports);

        txt += &line_end_subckt();

        txt
    }
}
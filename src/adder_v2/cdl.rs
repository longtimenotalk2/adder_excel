use crate::{adder_v2::{adder::Adder, cell_parse::Process}, hspice::{line_cap, line_inc}};

impl Adder {
    pub fn to_cdl(&self, name : &str, process : Process) -> String {
        let mut txt = String::new();

        // include
        for cell_body in self.cell_body_set() {
            txt += &line_inc(&cell_body.spf_path(process))
        }

        txt
    }
}
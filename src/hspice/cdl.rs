use crate::{cell_parse::{ProcessAndProject, RealCell}, hspice::{adder_pins_std, line_end_subckt, line_subckt}, std::adder::Adder};

impl Adder {
    pub fn to_cdl_std(&self, process : ProcessAndProject, name : &str) -> String {
        let mut txt = String::new();

        // inc cells
        for abstract_cells in &self.all_abstract_cells() {
            txt += &RealCell::parse(process, abstract_cells).line_inc();
        }

        txt += "\n";

        // subckt
        let ports = adder_pins_std(self.bits);
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
}
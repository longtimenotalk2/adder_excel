use std::collections::BTreeSet;

use crate::{cell_parse::{ProcessAndProject, RealCell}, std::adder::{AbstractCell, Adder, CustomDemand}};

impl Adder {
    pub fn l2h_abstract_cells(&self) -> BTreeSet<AbstractCell> {
        let mut ret = BTreeSet::new();
        for cell in &self.cells {
            if let Some(CustomDemand::DualVdd(dual_vdd_demand)) = cell.custom_demand.get(0) {
                if dual_vdd_demand.is_l2h() {
                    ret.insert(AbstractCell {
                        logic_block : cell.logic_block_map.logic_block.clone(),
                        drive : cell.drive.clone(),
                        custom_demand : cell.custom_demand.clone(),
                    });
                }
            }
        }

        ret
    }

    pub fn l2h_original_realcells(&self, process : ProcessAndProject) -> BTreeSet<RealCell> {
        let mut ret = BTreeSet::new();
        for abstract_cell in self.l2h_abstract_cells() {
            let original_cell = AbstractCell {
                logic_block : abstract_cell.logic_block.clone(),
                drive : abstract_cell.drive.clone(),
                custom_demand : vec![],
            };
            let real_cell = RealCell::parse(process, &original_cell);
            ret.insert(real_cell);
        }
        ret
    }

    pub fn l2h_original_realcell_spf_paths(&self, process : ProcessAndProject) -> Vec<String> {
        let mut ret = vec![];
        for real_cell in self.l2h_original_realcells(process) {
            ret.push(real_cell.inc_path());
        }
        
        ret
    }

    pub fn count_l2h(&self) -> i32 {
        let mut ret = 0;
        for cell in &self.cells {
            if let Some(CustomDemand::DualVdd(dual_vdd_demand)) = cell.custom_demand.get(0) {
                if dual_vdd_demand.is_l2h() {
                    ret += 1;
                }
            }
        }
        ret
    }

    pub fn list_l2h_inst(&self) -> Vec<String> {
        let mut ret = vec![];
        for cell in &self.cells {
            if let Some(CustomDemand::DualVdd(dual_vdd_demand)) = cell.custom_demand.get(0) {
                if dual_vdd_demand.is_l2h() {
                    ret.push(cell.inst_name());
                }
            }
        }
        ret
    }

    pub fn count_h2h(&self) -> i32 {
        let mut ret = 0;
        for cell in &self.cells {
            if let Some(CustomDemand::DualVdd(dual_vdd_demand)) = cell.custom_demand.get(0) {
                if dual_vdd_demand.is_h2h() {
                    ret += 1;
                }
            }
        }
        ret
    }
}
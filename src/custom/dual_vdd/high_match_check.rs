use std::collections::{BTreeMap, BTreeSet};

use crate::std::{adder::{AbstractCell, Adder, CellFullInfoInAdder, CustomDemand}, logic_block::Port, wire::Wire};

impl Adder {
    fn high_wires(&self) -> Vec<Wire> {
        let mut high_wires = vec![];
        for cell in &self.cells {
            if let Some(CustomDemand::DualVdd(dual_vdd_demand)) = cell.custom_demand.get(0) {
                if dual_vdd_demand.out_is_high() {
                    for (_port, wire) in &cell.logic_block_map.outputs {
                        high_wires.push(wire.clone());
                    }
                }
            }
        }
        high_wires
    }

    pub fn high_match_check(&self) {
        // 确保H2H的所有输入均为high，如果不符合则报错
        // 抓取所有high wire
        let high_wires =self.high_wires();
        // 检查每一个H2H是否输入的wire都在列表中
        let mut count = 0;
        let mut error = 0;
        for cell in &self.cells {
            if let Some(CustomDemand::DualVdd(dual_vdd_demand)) = cell.custom_demand.get(0) {
                if dual_vdd_demand.is_h2h() {
                    count += 1;
                    let mut errer_list = vec![];
                    for (_port, wire) in &cell.logic_block_map.inputs {
                        if !high_wires.contains(wire) {
                            errer_list.push(wire.clone());
                        }
                    }
                    
                    if !errer_list.is_empty() {
                        println!("ERROR : cell {:?} {:?} as H2H, input wire {:?} is not high!", cell.inst_name(), cell.logic_block_map.logic_block, errer_list);
                        error += 1;
                    }
                }
            }
        }
        println!("error / H2H count = {} / {}", error, count);
    }

    pub fn high_to_low_map(&self) -> BTreeMap<usize, BTreeSet<(Port, Wire)>> {
        let high_wires =self.high_wires();
        let mut ret = BTreeMap::new();
        // 检查每一个非H电源的前置哪些是高
        for (i, cell) in self.cells.iter().enumerate() {
            if cell.custom_demand.len() == 0 {
                let mut high_input_wires = BTreeSet::new();
                for (port, wire) in &cell.logic_block_map.inputs {
                    if high_wires.contains(wire) {
                        high_input_wires.insert((port.clone(), wire.clone()));
                    }
                }
                if high_input_wires.len() > 0 {
                    ret.insert(i, high_input_wires);
                }
            }
        }

        ret
    }
}
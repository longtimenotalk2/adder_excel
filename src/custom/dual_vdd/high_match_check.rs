use crate::std::adder::{Adder, CustomDemand};

impl Adder {
    pub fn high_match_check(&self) {
        // 确保H2H的所有输入均为high，如果不符合则报错
        // 抓取所有high wire
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
}
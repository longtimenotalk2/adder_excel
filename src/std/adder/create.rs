use colorful::{Color, Colorful};

use crate::std::{adder::{Adder, CellFullInfoInAdder, CellHinter, CustomDemand, Drive}, logic_block::LogicBlock, node_create::LogicBlockMappingTable, wire::{Flag, Wire}};

impl Adder {
    pub fn create_by_cell_hint(
        bits : usize,
        input_is_neg : bool,
        output_is_neg : bool,
        hints : Vec<CellHinter>, // layer
        end_xnr_not_new_q : Vec<usize>, // 指定index的q不找最新的
        end_xnr_not_new_g : Vec<usize>, // 指定index的g不找最新的
    ) -> Self {
        let mut cells = vec![];
        let mut history_wires = vec![];
        for i in 0..bits {
            history_wires.push(Wire::from_str(&format!("a{i}")));
            history_wires.push(Wire::from_str(&format!("b{i}")));
        }
        for hint in hints {
            // dbg!(&hint);
            let wire = hint.wire_ref;
            let layer = hint.layer;
            let drive = hint.drive;
            let custom_demand = hint.custom_demand;

            let mut error_infos = String::new();
            let mut result = None;
            for logic_block_hint in hint.logic_block_hints {
                match LogicBlockMappingTable::create_from_wire_by_hint_and_custom_demand(
                    &wire,
                    &logic_block_hint,
                    &history_wires,
                    &custom_demand
                ) {
                    Ok(r) => {
                        result = Some(r);
                        break;
                    },
                    Err(e) => {
                        error_infos += &format!("{:?}\n", e);
                    }
                }
            }
            if let Some(logic_block_map) = result.clone() {
                cells.push(CellFullInfoInAdder {
                    logic_block_map,
                    drive,
                    custom_demand,
                    layer,
                    index : wire.index,
                });
            } else {
                dbg!(&history_wires);
                panic!("\n\nwhen create wire {} at layer {layer} :\n\n {}", format!("{wire:?}").color(Color::Yellow), error_infos);
            }
            let mut actual_wires = vec![];
            for a in result.unwrap().outputs.values() {
                actual_wires.push(a.clone());
            }

            history_wires.append(&mut actual_wires);
        }

        // assert_eq!(input_is_neg, false);

        // 先假设输入是p，然后给出对应的输出极性
        let output_is_neg_old = output_is_neg;
        let output_is_neg = if input_is_neg { !output_is_neg } else { output_is_neg };

        // 检查当前的S是否都是符合输出的
        let mut has_s = vec![];
        for wire in &history_wires {
            if wire.flag == Flag::S {
                has_s.push(wire.index);
            }
        }
        if !has_s.contains(&0) {
            cells.push(CellFullInfoInAdder {
                logic_block_map : LogicBlockMappingTable::new_from_vec(
                    LogicBlock::INV, 
                    vec![Wire {flag : Flag::Q, index : 0, len : 1, is_neg : !output_is_neg}], 
                    vec![Wire {flag : Flag::S, index : 0, len : 1, is_neg : false}],
                ),
                drive : Drive::D1,
                custom_demand : vec![],
                layer : 10,
                index : 0,
            });
        }
        for index in 1..bits {
            if !has_s.contains(&index) {
                // 寻找最新的q或者nq
                let mut wire_q = None;
                let mut count = 0;
                for wire in history_wires.iter().rev() {
                    if wire.flag == Flag::Q && wire.index == index && wire.len == 1 {
                        if end_xnr_not_new_q.contains(&index) {
                            if count == 0 {
                                count += 1;
                            } else {
                                wire_q = Some(wire.clone());
                                break;
                            }

                        } else {
                            wire_q = Some(wire.clone());
                            break;
                        }
                        
                    }
                }
                let wire_q = wire_q.expect(&format!("can not find q for index {} in adder", index));
                // 寻找最新的g
                let mut wire_g = None;
                let mut count = 0;
                for wire in history_wires.iter().rev() {
                    if wire.flag == Flag::G && wire.index == index - 1 && wire.len == index {
                        if end_xnr_not_new_g.contains(&index) {
                            if count == 0 {
                                count += 1;
                            } else {
                                wire_g = Some(wire.clone());
                                break;
                            }
                        } else {
                            wire_g = Some(wire.clone());
                            break;
                        }
                        
                    }
                }
                let wire_g = wire_g.expect(&format!("can not find g{}_0 for in adder", index -1));

                let use_xnr = wire_q.is_neg ^ wire_g.is_neg ^ output_is_neg;
                let logic_block = if use_xnr {
                    LogicBlock::XNR2
                } else {
                    LogicBlock::XOR2
                };
                let wire_s = Wire {
                    flag : Flag::S,
                    index,
                    len   : 1,
                    is_neg: false,
                };
                cells.push(CellFullInfoInAdder {
                    logic_block_map : LogicBlockMappingTable::new_from_vec(
                        logic_block, 
                        vec![wire_g, wire_q], // A1是更快的，输入g
                        vec![wire_s.clone()],
                    ),
                    drive : Drive::D1,
                    custom_demand : vec![], 
                    layer : 10,
                    index,
                });
                history_wires.push(wire_s);
            }
        }

        
        let mut adder = Self {
            bits,
            input_is_neg,
            output_is_neg : output_is_neg_old,
            cells,
            // wires : history_wires,
        };

        if input_is_neg {
            adder = adder.all_cell_rev();
        }

        adder
    }

    pub fn all_cell_rev(mut self) -> Self {
        // 将所有cell使用其对偶cell

        for cell in &mut self.cells {
            *cell = cell.clone().rev();
        }

        self
    }
}

impl CellFullInfoInAdder {
    pub fn rev(mut self) -> Self {
        self.logic_block_map =  self.logic_block_map.rev();

        self
    }
}
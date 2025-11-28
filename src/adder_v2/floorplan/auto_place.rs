/*
首先给定总行数
然后按照cell顺序往里丢cell，找到34种情况中代价最小的点
*/

pub mod draw;

use std::collections::BTreeMap;

use crate::adder_v2::{Id, adder::Adder, cell::Cell, excel::{ExcelFrame, excel_to_datalist::ExcelDataList}, wire::Wire};

#[derive(Debug, Clone)]
struct PlacePos {
    row : i32,
    col : i32,
}


#[derive(Debug, Clone)]
pub struct CellPlaceInfo {
    pos : PlacePos,
    width : i32,
    contain_wire_list : Vec<(Id, Wire)>,
    left_direct_wire_list : Vec<(Id, Wire)>,
}

#[derive(Debug, Clone)]
pub struct PlaceData {
    pub data : BTreeMap<Id, CellPlaceInfo>,
    pub cells : Vec<(Id, Cell)>,
    pub row_num : i32,
    pub input_wire_row : BTreeMap<(Id, Wire), i32>,
    pub output_wire_row : BTreeMap<(Id, Wire), i32>,
}


impl PlaceData {
    pub fn new(row_num : i32, cells : Vec<(Id, Cell)>) -> Self {
        Self {
            data : BTreeMap::new(),
            cells,
            row_num,
            input_wire_row : BTreeMap::new(),
            output_wire_row : BTreeMap::new(),
        }
    }

    fn right_most_col_with_row(&self, row : i32) -> i32 {
        let mut col = 0;
        for (_, cell) in self.data.iter() {
            if cell.pos.row == row {
                col = col.max(cell.pos.col + cell.width);
            }
        }

        col
    }

    fn cost_from_cell_to_pos(&self, cell_id: Id, pos : &PlacePos, target_width : i32) -> i32 {
        let cell_row = self.data.get(&cell_id).unwrap().pos.row;
        let cell_col = self.data.get(&cell_id).unwrap().pos.col;
        let cell_width = self.data.get(&cell_id).unwrap().width;
        let target_row = pos.row;
        let target_col = pos.col;

        if cell_row == target_row && cell_col + cell_width == target_col {
            // M0 直连
            0
        } else {
            let row_abs_diff = (cell_row - target_row).abs();
            let col_abs_diff = if cell_col + cell_width - 1 < target_col {
                (cell_col + cell_width - 1 - target_col).abs()
            } else if cell_col > target_col + target_width - 1{
                (cell_col - (target_col + target_width - 1)).abs()
            } else {
                0
            };
            row_abs_diff * 156 + col_abs_diff * 48
        }
    }

    fn cost_from_input(&self, input_row: i32,  pos : &PlacePos) -> i32 {
        let target_row = pos.row;
        let target_col = pos.col;

        if target_row == input_row && target_col == 0 {
            // M0 直连
            0
        } else {
            let row_abs_diff = (input_row - target_row).abs();
            let col_abs_diff = target_col;
            row_abs_diff * 156 + col_abs_diff * 48
        }
    }

    // 给出cost和直连的wire
    fn try_set_given_row_most_left_and_give_cost(&self, row : i32, input_wire_list : &[(Id, Wire)], index_max : usize) -> (i32, Vec<(Id, Wire)>) {
        let col = self.right_most_col_with_row(row);
        let pos = PlacePos { row, col };

        let mut wire_cost_dict: BTreeMap<(Id, Wire), Option<i32>> = BTreeMap::new();
        for wire in input_wire_list.iter() {
            let value = self.input_wire_row.get(wire).map(|input_row| self.cost_from_input(*input_row, &pos));
            wire_cost_dict.insert(wire.clone(), value);
        }


        for (cell_id, cell) in self.data.iter() {
            for wire in cell.contain_wire_list.iter() {
                if input_wire_list.contains(wire) {
                    let mut cost = self.cost_from_cell_to_pos(*cell_id, &pos, cell.width);
                    // 增添结束位置修正
                    fn adder_index_to_row(index : usize) -> i32 {
                        if index <= 14 {
                            index as i32 + 1
                        } else {
                            index as i32 + 3
                        }
                    }
                    cost += {
                        let out_row = adder_index_to_row(index_max);
                        let row_diff = (out_row - row).abs();
                        row_diff * 155
                    };
                    wire_cost_dict.entry(wire.clone()).and_modify(|x| *x = x.map_or(Some(cost), |v| Some(v + cost)));
                }
            }
        }

        let mut direct_wires = vec![];
        let mut all_cost = 0;
        for (wire, wire_cost) in wire_cost_dict.iter() {
            all_cost += wire_cost.expect(&format!("can not find cost for wire {}", wire.1.to_string()));
            if wire_cost.unwrap() == 0 {
                direct_wires.push(wire.clone())
            }
        }

        (all_cost, direct_wires)
    }

    fn insert_cell_to_row_left(&mut self, cell_id : Id, row : i32, width : i32, contain_wire_list : Vec<(Id, Wire)>, left_direct_wire_list : Vec<(Id, Wire)>) {
        let col = self.right_most_col_with_row(row);
        let pos = PlacePos { row, col };
        self.data.insert(cell_id, CellPlaceInfo { pos, width, contain_wire_list, left_direct_wire_list});
    }
}

impl Adder {
    pub fn auto_place(&self) -> PlaceData {
        let mut place_data = PlaceData::new(34, self.cells.clone());

        fn adder_index_to_row(index : usize) -> i32 {
            if index <= 14 {
                index as i32 + 1
            } else {
                index as i32 + 3
            }
        }

        for wire in self.wires.iter() {
            if wire.1.is_input() {
                place_data.input_wire_row.insert(wire.clone(), adder_index_to_row(wire.1.index));
            } else if wire.1.is_output() {
                place_data.output_wire_row.insert(wire.clone(), adder_index_to_row(wire.1.index));
            }
        }

        for (cell_id, cell) in self.cells.iter() {
            let width = cell.to_cell_body().width();
            let input_wire_list : Vec<(Id, Wire)> = cell.node.io.input.clone().into_iter().map(|x| x.1).collect();
            let mut output_wire_list : Vec<(Id, Wire)> = vec![cell.node.io.output_z.clone()];
            if let Some(output) = &cell.node.io.output_o1 {
                output_wire_list.push(output.clone());
            }
            let contain_wire_list = [input_wire_list.clone(), output_wire_list.clone()].concat();

            let index = cell.node.io.output_z.1.index;

            let mut best_row = 0;
            let  (mut best_cost, mut best_direct_wires) = place_data.try_set_given_row_most_left_and_give_cost(0, &input_wire_list, index);
            for row in 1..=34 {
                let (cost, direct_wires) = place_data.try_set_given_row_most_left_and_give_cost(row, &input_wire_list, index);
                if cost < best_cost {
                    best_cost = cost;
                    best_row = row;
                    best_direct_wires = direct_wires;
                }
            }

            place_data.insert_cell_to_row_left(*cell_id, best_row, width, contain_wire_list, best_direct_wires);

            println!("cell {} place at row {best_row} : cost = {best_cost}", cell.to_string())
        }



        place_data
    }
}


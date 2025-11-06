pub mod draw_logic;
pub mod draw_cell;
pub mod draw_main;

use std::collections::BTreeMap;
use svg::{node::element::{Circle, Rectangle, Text}, Document, Node};

use crate::adder_v2::{adder::Adder, draw::adder_frame::{AdderFrame, CellPos, Pos, WirePos}, excel::{excel_to_datalist::ExcelDataList, ExcelFrame}, Id};

pub struct AdderDraw {
    pub cell_width : f32,
    pub cell_height : f32,
    pub cell_x_interval : f32,
    pub cell_y_interval : f32,
    pub wire_x_interval : f32,
    pub wire_height : f32,
    pub border_up : f32,
    pub border_down : f32,
    pub border_left : f32,
    pub border_right : f32,
    pub font_index : f32,
    pub font_cell_name : f32,
    pub font_wire_name : f32,
    pub wire_line_width : f32,
    pub show_vddh : bool,
}

impl AdderDraw {
    pub fn new() -> Self {
        Self {
            cell_width: 60.,
            cell_height: 30.,
            cell_x_interval: 20.0,
            cell_y_interval: 100.0,
            wire_x_interval: 25.0,
            wire_height: 20.0,
            border_up: 100.0,
            border_down: 100.0,
            border_left: 100.0,
            border_right: 100.0,
            font_index: 20.,
            font_cell_name: 12.,
            font_wire_name: 10.,
            wire_line_width: 1.,
            show_vddh: true,
        }
    }
}

pub struct BigRuler {
    full_height : f32,
    full_width : f32,
    cell_data : BTreeMap<Pos, BTreeMap<CellPos, (f32, f32)>>, // 记录某个整数坐标下，根据Cell Pos返回cell的绝对坐标
    wire_data : BTreeMap<Pos, BTreeMap<CellPos, BTreeMap<WirePos, (f32, f32)>>>, // 记录某个整数坐标下，根据Cell Pos以及Wire Pose，返回wire的绝对坐标
}

impl BigRuler {
    pub fn get_cell_xy(&self, pos : &Pos, cell_pos : &CellPos) -> (f32, f32) {
        self.cell_data.get(pos).unwrap().get(cell_pos).unwrap().clone()
    }

    pub fn get_wire_xy(&self, pos : &Pos, cell_pos : &CellPos, wire_pos : &WirePos) -> (f32, f32) {
        self.wire_data.get(pos).unwrap().get(cell_pos).unwrap().get(wire_pos).unwrap().clone()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ToBeDraw {
    pub front : Vec<Box<dyn Node>>,
    pub back : Vec<Box<dyn Node>>,
}

impl ToBeDraw {
    pub fn new(front : Vec<Box<dyn Node>>, back : Vec<Box<dyn Node>>) -> Self {
        Self {
            front,
            back,
        }
    }
    pub fn update(&mut self, other : Self) {
        self.front.extend(other.front);
        self.back.extend(other.back);
    }
}

impl AdderDraw {
    pub fn get_big_ruler(&self, frame : &AdderFrame) -> BigRuler {
        let bits = frame.bits;
        
        let mut max_cell_len_with_index = vec![1; frame.bits];

        for (pos, cells) in frame.frame.iter() {
            let index = pos.index;
            max_cell_len_with_index[index] = cells.len().max(max_cell_len_with_index[index]);
        }


        let layer_max = frame.layer_max;


        let mut full_width = (bits + 1) as f32 * self.cell_x_interval;
        for cell_len in max_cell_len_with_index.iter() {
            full_width += *cell_len as f32 * self.cell_width;
        }

        let full_height = (layer_max + 2) as f32 * (self.cell_y_interval + self.cell_height) - self.cell_y_interval * 2.;

        let mut cell_data : BTreeMap<Pos, BTreeMap<CellPos, (f32, f32)>> = BTreeMap::new();
        let mut wire_data : BTreeMap<Pos, BTreeMap<CellPos, BTreeMap<WirePos, (f32, f32)>>> = BTreeMap::new();

        // 输入的wire data
        let y = self.cell_height / 2.0 + self.border_up;
        let y_wire = y + self.cell_height / 2.0 + self.wire_height;
        
        for index in 0..frame.bits {
            let x = full_width - ( 
                max_cell_len_with_index[0..=index].iter().sum::<usize>() as f32 * self.cell_width + (index + 1) as f32 * self.cell_x_interval + self.cell_width / 2.0
            ) + self.border_left + self.cell_width;
            let pos = Pos::new(index, 0);
            let mut inner = BTreeMap::new();
            inner.insert(WirePos::default(), (x - self.wire_x_interval / 2.0 , y_wire));
            inner.insert(WirePos::new(1), (x + self.wire_x_interval / 2.0, y_wire));
            wire_data.entry(pos.clone()).or_default().insert(CellPos::default(), inner);
        }

        // cell 的out wire data

        for (pos, cells) in frame.frame.iter() {
            let index = pos.index;
            let layer = pos.layer;
            let y = (self.cell_y_interval + self.cell_height) * layer as f32 + self.cell_height / 2.0 + self.border_up;
            let y_wire = y + self.cell_height / 2.0 + self.wire_height;
            let mut x = full_width - ( 
                max_cell_len_with_index[0..=index].iter().sum::<usize>() as f32 * self.cell_width + (index + 1) as f32 * self.cell_x_interval + self.cell_width / 2.0
            ) + self.border_left;
            for (cell_pos, cell) in cells.iter().enumerate() {
                x += self.cell_width;
                let cell_pos = CellPos::new(cell_pos);
                cell_data.entry(pos.clone()).or_default().insert(cell_pos.clone(), (x, y));
                let wire_len = cell.outputs.len();
                match wire_len {
                    1 => {
                        let mut inner = BTreeMap::new();
                        inner.insert(WirePos::default(), (x, y_wire));
                        wire_data.entry(pos.clone()).or_default().insert(cell_pos.clone(), inner);
                    },
                    2 => {
                        let mut inner = BTreeMap::new();
                        inner.insert(WirePos::default(), (x - self.wire_x_interval / 2.0 , y_wire));
                        inner.insert(WirePos::new(1), (x + self.wire_x_interval / 2.0, y_wire));
                        wire_data.entry(pos.clone()).or_default().insert(cell_pos.clone(), inner);
                    },
                    _ => unimplemented!(),
                }
            }
        }

        BigRuler {
            full_height,
            full_width,
            cell_data,
            wire_data,
        }
    }

    
}

pub mod draw_logic;

use std::collections::BTreeMap;

use macroquad::prelude::*;
use svg::{node::element::{Rectangle, Text}, Document};

use crate::adder_v2::{adder::Adder, draw::adder_frame::{AdderFrame, CellPos, Pos, WirePos}, excel::{excel_to_datalist::ExcelDataList, ExcelFrame}, Id};

pub struct AdderDraw {
    pub cell_width : f32,
    pub cell_height : f32,
    pub cell_x_interval : f32,
    pub cell_y_interval : f32,
    pub wire_x_interval : f32,
}

impl AdderDraw {
    pub fn new() -> Self {
        Self {
            cell_width: 50.,
            cell_height: 30.,
            cell_x_interval: 20.0,
            cell_y_interval: 20.0,
            wire_x_interval: 20.0,
        }
    }
}

pub struct BigRuler {
    full_height : f32,
    full_width : f32,
    data : BTreeMap<Pos, BTreeMap<CellPos, BTreeMap<WirePos, (f32, f32)>>>
}

impl AdderDraw {
    pub fn get_big_ruler(&self, frame : &AdderFrame) -> BigRuler {
        let bits = frame.bits;
        
        let mut max_cell_len_with_index = vec![1; frame.bits];
        let mut layer_max = 0;
        for (pos, cells) in frame.frame.iter() {
            let index = pos.index;
            max_cell_len_with_index[index] = cells.len().max(max_cell_len_with_index[index]);
            layer_max = layer_max.max(pos.layer);
        }

        let mut full_width = (bits + 1) as f32 * self.cell_x_interval;
        for cell_len in max_cell_len_with_index.iter() {
            full_width += *cell_len as f32 * self.cell_width;
        }

        let full_height = (layer_max + 1) as f32 * self.cell_y_interval + layer_max as f32 * self.cell_height;

        for (pos, cells) in frame.frame.iter() {
            let index = pos.index;
            let layer = pos.layer;
            let y = (self.cell_y_interval + self.cell_height) * (layer + 1) as f32 -  self.cell_height / 2.0;
            let mut x = full_width - ( 
                max_cell_len_with_index[0..index].iter().sum::<usize>() as f32 * self.cell_width + (index + 1) as f32 * self.cell_x_interval + self.cell_width / 2.0
            );
            let mut pos_ruler : BTreeMap<CellPos, BTreeMap<WirePos, (f32, f32)>> = BTreeMap::new();
            for cell in cells.iter() {
                x += self.cell_width;

            }
        }

        todo!()
    }

    pub fn draw(&self, frame : &AdderFrame, save_path : &str) {
        let bits = frame.bits;
        // 初始数据处理
        let mut max_cell_len_with_index = vec![1; frame.bits];
        for (pos, cells) in frame.frame.iter() {
            let index = pos.index;
            max_cell_len_with_index[index] = cells.len().max(max_cell_len_with_index[index]);
        }

        let mut full_width = (bits + 1) as f32 * self.cell_x_interval;
        for cell_len in max_cell_len_with_index.iter() {
            full_width += *cell_len as f32 * self.cell_width;
        }

        // 创建白色背景矩形
        let background = Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", "white");

        let mut document = Document::new()
            .set("viewBox", (0, 0, full_width, 1000)) // 视口大小
            .add(background);

        let get_x = |index : usize, cell_pos : usize, wire_pos : usize| -> f32 {
            let mut x: f32 = 0.;
            x += (index + 1) as f32 * self.cell_x_interval;
            for i in 0..index {
                x += max_cell_len_with_index[i] as f32 * self.cell_width;
            }
            x
        };

        for i in 0..bits {
            let rec = Rectangle::new()
                .set("x", get_x(i, 0, 0))
                .set("y", 100)
                .set("width", self.cell_width)
                .set("height", self.cell_height)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("fill", "blue")
                .set("stroke", "black")
                .set("stroke-width", 2);
            document = document.add(rec);
        }

        let text = Text::new("AOAI211")
            .set("x", 50)
            .set("y", 50)
            .set("text-anchor", "middle") // 水平居中
            .set("dominant-baseline", "middle")   // 垂直居中
            .set("font-family", "Arial")
            .set("font-size", 10);
        document = document.add(text);

        svg::save(save_path, &document).unwrap();
    }
}

#[test]
fn test_frame() {
    const PATH : &'static str = "src/adder_v2/project/a01_same_vt_vddh/excel/b08_t03_pn.txt";

    fn adder()  -> Adder {
        adder_and_excel().0
    }

    fn adder_and_excel()  -> (Adder, ExcelDataList<Id>) {
        let excel_frame = ExcelFrame::load(PATH);
        let excel_data_list = ExcelDataList::from_excel_frame(&excel_frame);
        let (adder, excel_map) = Adder::create_from_excel_data_list(excel_data_list, false, true);
        adder.check_id_all_match();
        (adder, excel_map)
    }
    
    let frame = AdderFrame::from_adder(&adder());
    let draw = AdderDraw::new();
    draw.draw(&frame, "adder.svg");
}
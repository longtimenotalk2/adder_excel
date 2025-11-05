use macroquad::prelude::*;

use crate::adder_v2::{adder::Adder, draw::adder_frame::AdderFrame, excel::{excel_to_datalist::ExcelDataList, ExcelFrame}, Id};

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
            cell_width: 50.0,
            cell_height: 50.0,
            cell_x_interval: 10.0,
            cell_y_interval: 10.0,
            wire_x_interval: 10.0,
        }
    }
}

impl AdderDraw {
    pub fn draw(&self, frame : &AdderFrame) {
        // 初始数据处理
        let mut max_cell_len_with_index = vec![1; frame.bits];
        for (pos, cells) in frame.frame.iter() {
            let index = pos.index;
            max_cell_len_with_index[index] = cells.len().max(max_cell_len_with_index[index]);
        }

        dbg!(&max_cell_len_with_index);
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
    draw.draw(&frame);
}
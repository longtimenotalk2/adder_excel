use std::collections::BTreeMap;

pub mod load;
pub mod to_cell_hinter;
pub mod create_from_excel;
pub mod cap_print;

#[derive(Debug, Clone)]
enum LayerType {
    Cri,
    Uncri,
}

#[derive(Debug, Clone)]
struct Node {
    index : usize,
    layer : i32,
    wire_txt : String,
    code_txt : String,
}

#[derive(Debug, Clone)]
pub struct ExcelData {
    bits : usize,
    nodes : Vec<Node>,
    excel_layout_positions : Vec<(usize, usize)>, // 行，index
    excel_row_layer : Vec<i32>,
    excel_row_layer_type : Vec<LayerType>,
    excel_cap_data : BTreeMap<(usize, usize), Vec<i32>>,
}
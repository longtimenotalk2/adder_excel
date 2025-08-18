pub mod load;
pub mod to_cell_hinter;
pub mod create;

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
    layer_type : LayerType,
}

#[derive(Debug, Clone)]
pub struct ExcelData {
    bits : usize,
    nodes : Vec<Node>,
}
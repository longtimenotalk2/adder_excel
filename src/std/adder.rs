
use crate::std::{logic_block::LogicBlock, node_create::{LogiBlockHint, LogicBlockMappingTable}};

#[derive(Debug, Clone, Copy)]
pub enum Drive {
    D1,
    D2,
}

#[derive(Debug, Clone)]
pub struct Cell {
    logic_block_map : LogicBlockMappingTable,
    drive : Drive,
    custom_demand : CustomDemand,
}

#[derive(Debug, Clone)]
pub enum CustomDemand {}

#[derive(Debug, Clone)]
pub struct CellHinter {
    pub logic_block_hints : Vec<LogiBlockHint>,
    pub drive : Drive,
    pub custom_demand : Vec<CustomDemand>,
}

#[derive(Debug, Clone)]
pub struct Adder {
    bits : usize,
    input_is_neg : bool,
    output_is_neg : bool,
}
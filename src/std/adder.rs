
pub mod create;
pub mod function_check;

use crate::{custom::domino::DominoDemand, std::{logic_block::LogicBlock, node_create::{LogiBlockHint, LogicBlockMappingTable}, wire::Wire}};

#[derive(Debug, Clone, Copy)]
pub enum Drive {
    D1,
    D2,
}

#[derive(Debug, Clone)]
pub struct Cell {
    logic_block_map : LogicBlockMappingTable,
    drive : Drive,
    custom_demand : Vec<CustomDemand>,
    layer : i32,
    index : usize,
}

#[derive(Debug, Clone)]
pub enum CustomDemand {
    Domino(DominoDemand)
}

#[derive(Debug, Clone)]
pub struct CellHinter {
    pub logic_block_hints : Vec<LogiBlockHint>,
    pub drive : Drive,
    pub custom_demand : Vec<CustomDemand>,
    pub wire_ref : Wire,
    pub layer : i32,
}

#[derive(Debug, Clone)]
pub struct Adder {
    bits : usize,
    input_is_neg : bool,
    output_is_neg : bool,
    cells : Vec<Cell>,
    // wires : Vec<Wire>,
}
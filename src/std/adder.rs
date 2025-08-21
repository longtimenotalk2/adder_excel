
pub mod create;
pub mod function_check;
pub mod property;

use crate::{custom::domino::DominoDemand, std::{logic_block::LogicBlock, node_create::{LogiBlockHint, LogicBlockMappingTable}, wire::Wire}};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Drive {
    D1,
    D2,
}

#[derive(Debug, Clone)]
pub struct CellFullInfoInAdder {
    logic_block_map : LogicBlockMappingTable,
    drive : Drive,
    custom_demand : Vec<CustomDemand>,
    layer : i32,
    index : usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AbstractCell {
    pub logic_block : LogicBlock,
    pub drive : Drive,
    pub custom_demand : Vec<CustomDemand>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    cells : Vec<CellFullInfoInAdder>,
}
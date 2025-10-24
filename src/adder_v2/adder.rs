mod adder_create;

use crate::adder_v2::{node::Node, wire::{Wire}, Id};

#[derive(Debug, Clone)]
pub enum Drive {
    D1,
    D2,
}

impl Default for Drive {
    fn default() -> Self {
        Drive::D1
    }
}

#[derive(Debug, Clone, Default)]
pub struct CellInfo {
    pub drive : Drive
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub node: Node,
    pub info: CellInfo,
}

impl Cell {
    pub fn new(node: Node, info: CellInfo) -> Self {
        Self {
            node,
            info
        }
    }
}

#[derive(Debug, Clone)]
pub struct Adder {
    wires : Vec<(Id, Wire)>,
    cells : Vec<(Id, Cell)>,
    bits : usize,
}
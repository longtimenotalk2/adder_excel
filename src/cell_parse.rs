use std::collections::BTreeMap;

use crate::std::{logic_block::Port, wire::Wire};


pub mod n4c;
pub mod n4c_custom;

#[derive(Debug, Clone, Copy)]
pub enum CellSourceType {
    Std,
    Custom,
    Lhw,
}

#[derive(Debug, Clone, Copy)]
pub enum ProcessAndProject {
    N3E1374,
    N4C1340,
}

#[derive(Debug, Clone, Copy)]
pub enum PortType {
    Logic,
    PG,
}

#[derive(Debug, Clone)]
pub struct RealCell {
    name : String,
    source_type : CellSourceType,
    process : ProcessAndProject,
}
//ports : BTreeMap<Port, (Wire, PortType)>,
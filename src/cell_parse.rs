use std::collections::{BTreeMap, BTreeSet};

use crate::std::{adder::{AbstractCell, CustomDemand, Drive}, logic_block::{LogicBlock, Port}, wire::Wire};


pub mod n4c;
pub mod n4c_custom;
pub mod n3e;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CellSourceType {
    Std,
    Custom,
    Lhw,
    LocalHack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProcessAndProject {
    N3E1374,
    N4C1340,
    N4C1342H200,
}

#[derive(Debug, Clone, Copy)]
pub enum PortType {
    Logic,
    PG,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RealCell {
    pub name : String,
    pub source_type : CellSourceType,
    pub process : ProcessAndProject,
    pub addition_pg_port : BTreeSet<Port>,
    pub vdd_replaced : Vec<Port>,
}

impl RealCell {
    pub fn parse(
        process : ProcessAndProject,
        abstract_cell : &AbstractCell
    ) -> Self {
        match process {
            ProcessAndProject::N3E1374 => {
                Self::parse_n3e(abstract_cell)
            }
            ProcessAndProject::N4C1340 => {
                Self::parse_n4c(abstract_cell)
            }
            ProcessAndProject::N4C1342H200 => {
                Self::parse_n4c_1342_h200(abstract_cell)
            }
        }
    }
}
use std::collections::BTreeMap;

use crate::std::{adder::{CustomDemand, Drive}, logic_block::{LogicBlock, Port}, wire::Wire};


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
    pub name : String,
    pub source_type : CellSourceType,
    pub process : ProcessAndProject,
}

impl RealCell {
    pub fn parse(
        process : ProcessAndProject,
        logic_block : LogicBlock,
        drive : Drive,
        custom_demand : Vec<CustomDemand>,
    ) -> Self {
        match process {
            ProcessAndProject::N3E1374 => {
                todo!()
            }
            ProcessAndProject::N4C1340 => {
                Self::parse_n4c(logic_block, drive, custom_demand)
            }
        }
    }
}
use std::collections::BTreeSet;

use crate::{cell_parse::{CellSourceType, ProcessAndProject, RealCell}, custom::{custom_logic_block::CustomLogicBlock, domino::{self, DominoDemand, DominoPolar}}, std::{adder::{CustomDemand, Drive}, logic_block::{LogicBlock, Port}}};

impl RealCell {
    pub fn parse_n4c_custom(
        logic_block : &LogicBlock,
        drive : &Drive,
        custom_demand : &[CustomDemand],
    ) -> Self {
        assert_eq!(custom_demand.len(), 1);
        let name = match &custom_demand[0] {
            CustomDemand::Domino(domino) => {
                match (&domino.logic_block, &drive, &domino.polar, domino.is_start) {
                    (&LogicBlock::NR2, &Drive::D1, &DominoPolar::P, false) => "NR2D1_DOM_P_V06",
                    (&LogicBlock::OAI21, &Drive::D1, &DominoPolar::P, false) => "OAI21D1_DOM_P_V06",
                    (&LogicBlock::OAI21, &Drive::D2, &DominoPolar::P, false) => "OAI21D2_DOM_P_V06",
                    (&LogicBlock::NR2, &Drive::D1, &DominoPolar::P, true) => "NR2D1_DOM_P_START_V06",
                    (&LogicBlock::OAI21, &Drive::D1, &DominoPolar::P, true) => "OAI21D1_DOM_P_START_V06",
                    (&LogicBlock::AOI21, &Drive::D1, &DominoPolar::N, false) => "AOI21D1_DOM_N_V06",
                    (&LogicBlock::Custom(CustomLogicBlock::AOI2221), &Drive::D2, &DominoPolar::N, false) => "AOI2221D2_DOM_N_V06",
                    (&LogicBlock::Custom(CustomLogicBlock::NR4), &Drive::D1, &DominoPolar::N, true) => "NR4D1_DOM_N_START_V06",
                    (&LogicBlock::Custom(CustomLogicBlock::NR6), &Drive::D1, &DominoPolar::N, true) => "NR6D1_DOM_N_START_V06",
                    _ => panic!("{:?},{:?},{:?},is_start = {} is not implemented", domino.logic_block, drive, domino.polar, domino.is_start)
                }
            },
            // _ => unimplemented!()
        };
        let addition_pg_port  = match &custom_demand[0] {
            CustomDemand::Domino(domino) => {
                match &domino.polar {
                    &DominoPolar::P => "KN",
                    &DominoPolar::N => "K",
                }
            },
        };
        Self {
            name: name.to_string(),
            source_type : CellSourceType::Lhw,
            process : ProcessAndProject::N4C1340,
            addition_pg_port : BTreeSet::from([Port::new(addition_pg_port)])
        }
    }
}
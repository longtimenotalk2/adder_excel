use core::panic;
use std::collections::BTreeSet;

use crate::{cell_parse::{CellSourceType, ProcessAndProject, RealCell}, custom::{custom_logic_block::CustomLogicBlock, domino::{self, DominoDemand, DominoPolar}}, std::{adder::{AbstractCell, CustomDemand, Drive}, logic_block::{LogicBlock, Port}}};

impl RealCell {
    pub fn parse_n3e_custom(
        logic_block : &LogicBlock,
        drive : &Drive,
        custom_demand : &[CustomDemand],
    ) -> Self {
        assert_eq!(custom_demand.len(), 1);
        let name = match &custom_demand[0] {
            CustomDemand::Domino(domino) => {
                todo!()
            },
            CustomDemand::Gdi => {
                todo!()
            },
            CustomDemand::DualVdd(dual_vdd) => {
                if dual_vdd.is_l2h() {
                    match drive {
                        Drive::D1 => {
                            match logic_block {
                                LogicBlock::AOI21 => "AOI21D1BM156H3P48CPDELVT_P_ULVT_L2H_V03",
                                LogicBlock::AOI22 => "AOI22D1BM156H3P48CPDELVT_1_P_ULVT_L2H_V03",
                                LogicBlock::INV => "INVD1BM156H3P48CPDELVT_1_P_ULVT_L2H_V03",
                                LogicBlock::ND2 => "ND2D1BM156H3P48CPDELVT_1_P_ULVT_L2H_V03",
                                LogicBlock::NR2 => "NR2D1BM156H3P48CPDELVT_1_P_ULVT_L2H_V02",
                                LogicBlock::OAI21 => "OAI21D1BM156H3P48CPDELVT_P_ULVT_L2H_V02",
                                LogicBlock::OAI22 => "OAI22D1BM156H3P48CPDELVT_P_ULVT_L2H_V02",
                                _ => panic!("{logic_block:?} D1 did not impl L2H")
                            }
                        }
                        Drive::D2 => {
                            match logic_block {
                                LogicBlock::AOI21 => "AOI21D2BM156H3P48CPDELVT_P_ULVT_L2H_V03",
                                LogicBlock::ND2 => "ND2D2BM156H3P48CPDELVT_1_P_ULVT_L2H_V03",
                                LogicBlock::NR2 => "NR2D2BM156H3P48CPDELVT_1_P_ULVT_L2H_V02",
                                LogicBlock::OAI21 => "OAI21D2BM156H3P48CPDELVT_P_ULVT_L2H_V02",
                                _ => unimplemented!()
                            }
                        }
                    }
                } else if dual_vdd.is_h2h() {
                    match drive {
                        Drive::D1 => {
                            match logic_block {
                                LogicBlock::AOI21 => "AOI21D1BM156H3P48CPDELVT_H2H_V03",
                                LogicBlock::IAOI21 => "IAOI21D1BM156H3P48CPDELVT_H2H_V03",
                                LogicBlock::INV => "INVD1BM156H3P48CPDELVT_1_H2H_V03",
                                LogicBlock::IOAI21 => "IOAI21D1BM156H3P48CPDELVT_H2H_V02",
                                LogicBlock::ND2 => "ND2D1BM156H3P48CPDELVT_1_H2H_V03",
                                LogicBlock::NR2 => "NR2D1BM156H3P48CPDELVT_1_H2H_V02",
                                LogicBlock::OAI21 => "OAI21D1BM156H3P48CPDELVT_H2H_V02",
                                
                                _ => unimplemented!()
                            }
                        }
                        Drive::D2 => {
                            match logic_block {
                                LogicBlock::AOI21 => "AOI21D2BM156H3P48CPDELVT_H2H_V03",
                                LogicBlock::OAI21 => "OAI21D2BM156H3P48CPDELVT_H2H_V02",
                                _ => unimplemented!()
                            }
                        }
                    }
                } else {
                    unimplemented!()
                }

            }
        };
        match &custom_demand[0] {
            CustomDemand::Domino(domino) => {
                todo!()
            },
            CustomDemand::Gdi => {
                todo!()
            }
            CustomDemand::DualVdd(dual_vdd) => {
                let mut addition_pg_port = BTreeSet::new();
                if dual_vdd.out_is_high() {
                    addition_pg_port.insert(Port("VDDH".to_string()));
                }
                Self {
                    name: name.to_string(),
                    source_type : CellSourceType::Lhw,
                    process : ProcessAndProject::N3E1374,
                    addition_pg_port : addition_pg_port,
                    vdd_replaced : vec![],
                }
            }
        }
        
    }
}
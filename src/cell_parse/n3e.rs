use std::collections::BTreeSet;

use crate::{cell_parse::{CellSourceType, ProcessAndProject, RealCell}, custom, std::{adder::{AbstractCell, CustomDemand, Drive}, logic_block::LogicBlock}};

impl RealCell {
    pub fn parse_n3e(
        abstract_cell : &AbstractCell
    ) -> Self {
        let custom_demand = &abstract_cell.custom_demand;
        let drive = &abstract_cell.drive;
        let logic_block = &abstract_cell.logic_block;
        if custom_demand.len() == 0 {
            let (name, source_type) = match drive {
                Drive::D1 => { match logic_block {
                    LogicBlock::INV => ("INVD1BM156H3P48CPDELVT_1", CellSourceType::Std),
                    LogicBlock::ND2 => ("ND2D1BM156H3P48CPDELVT_1", CellSourceType::Std),
                    LogicBlock::NR2 => ("NR2D1BM156H3P48CPDELVT_1", CellSourceType::Std),
                    LogicBlock::AN2 => ("AN2D1BM156H3P48CPDELVT_1", CellSourceType::Std),
                    LogicBlock::OR2 => ("OR2D1BM156H3P48CPDELVT_1", CellSourceType::Std),
                    LogicBlock::IND2 => ("IND2D1BM156H3P48CPDELVT_1", CellSourceType::Std),
                    LogicBlock::INR2 => ("INR2D1BM156H3P48CPDELVT", CellSourceType::Custom),
                    LogicBlock::XOR2 => ("XOR2D1BM156H3P48CPDELVT", CellSourceType::Custom),
                    LogicBlock::XNR2 => ("XNR2D1BM156H3P48CPDELVT", CellSourceType::Custom),
                    LogicBlock::XOR2DOUT => ("XOR2D1_DUAL_OUT_BM156H3P48CPDELVT_ELVT_V1", CellSourceType::Custom),
                    LogicBlock::XNR2DOUT => ("XNR2D1_DUAL_OUT_BM156H3P48CPDELVT_ELVT_V1", CellSourceType::Custom),
                    LogicBlock::AOI21 => ("AOI21D1BM156H3P48CPDELVT", CellSourceType::Custom),
                    LogicBlock::OAI21 => ("OAI21D1BM156H3P48CPDELVT", CellSourceType::Custom),
                    LogicBlock::AO21 => ("AO21D1BM156H3P48CPDELVT", CellSourceType::Custom),
                    LogicBlock::OA21 => ("OA21D1BM156H3P48CPDELVT", CellSourceType::Custom),
                    LogicBlock::IAOI21 => ("IAOI21D1BM156H3P48CPDELVT", CellSourceType::Custom),
                    LogicBlock::IOAI21 => ("IOAI21D1BM156H3P48CPDELVT", CellSourceType::Custom),
                    LogicBlock::AOI22 => ("AOI22D1BM156H3P48CPDELVT", CellSourceType::Custom),
                    LogicBlock::OAI22 => ("OAI22D1BM156H3P48CPDELVT", CellSourceType::Custom),
                    LogicBlock::AOAI211 => ("AOAI211D1BM156H3P48CPDELVT", CellSourceType::Custom),
                    LogicBlock::OAOI211 => ("OAOI211D1BM156H3P48CPDELVT", CellSourceType::Custom),
                    LogicBlock::AOA211 => ("AOA211D1BM156H3P48CPDELVT", CellSourceType::Custom),
                    LogicBlock::OAO211 => ("OAO211D1BM156H3P48CPDELVT", CellSourceType::Custom),
                    LogicBlock::Custom(..) => unimplemented!()
                }}
                Drive::D2 => { match logic_block {
                    LogicBlock::INV => ("INVD2BM156H3P48CPDELVT_1", CellSourceType::Std),
                    LogicBlock::AOI21 => ("AOI21D2BM156H3P48CPDELVT", CellSourceType::Custom),
                    LogicBlock::OAI21 => ("OAI21D2BM156H3P48CPDELVT", CellSourceType::Custom),
                    LogicBlock::ND2 => ("ND2D2BM156H3P48CPDELVT_1", CellSourceType::Std),
                    LogicBlock::NR2 => ("NR2D2BM156H3P48CPDELVT_1", CellSourceType::Std),
                    LogicBlock::IND2 => ("IND2D2BM156H3P48CPDELVT_1", CellSourceType::Std),
                    LogicBlock::AOI22 => ("AOI22D2BM156H3P48CPDELVT_1", CellSourceType::Std),
                    _ => panic!("{logic_block:?} dont impl D2")
                }}
            };
            Self {
                name : name.to_string(),
                source_type,
                process : ProcessAndProject::N4C1340,
                addition_pg_port : BTreeSet::new(),
                vdd_replaced : vec![],
            }
        } else {
            Self::parse_n4c_custom(logic_block, drive, custom_demand)
        }
    }

    
}


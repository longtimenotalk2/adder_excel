use crate::{cell_parse::{CellSourceType, ProcessAndProject, RealCell}, std::{adder::{CustomDemand, Drive}, logic_block::LogicBlock}};

impl RealCell {
    fn parse_n4c(
        logic_block : LogicBlock,
        drive : Drive,
        custom_demand : Vec<CustomDemand>,
    ) -> Self {
        let (name, source_type) = if custom_demand.len() == 0 {
            match drive {
                Drive::D1 => { match logic_block {
                    LogicBlock::INV => ("INVMZD1BWP200H6P51CNODELVT", CellSourceType::Std),
                    LogicBlock::ND2 => ("ND2MZD1BWP200H6P51CNODELVT", CellSourceType::Std),
                    LogicBlock::NR2 => ("NR2MZD1BWP200H6P51CNODELVT", CellSourceType::Std),
                    LogicBlock::AN2 => ("AN2MZD1BWP200H6P51CNODELVT", CellSourceType::Std),
                    LogicBlock::OR2 => ("OR2MZD1BWP200H6P51CNODELVT", CellSourceType::Std),
                    LogicBlock::IND2 => ("IND2MZD1BM200H6P51CNODELVT", CellSourceType::Custom),
                    LogicBlock::INR2 => ("INR2MZD1BM200H6P51CNODELVT", CellSourceType::Custom),
                    LogicBlock::XOR2 => ("XOR2SAMZD1BM200H6P51CNODELVT", CellSourceType::Custom),
                    LogicBlock::XNR2 => ("XNR2SAMZD1BM200H6P51CNODELVT", CellSourceType::Custom),
                    LogicBlock::XOR2DOUT => ("XOR2SAMZD1_DUAL_OUT_BM200H6P51CNODELVT", CellSourceType::Custom),
                    LogicBlock::XNR2DOUT => ("XNR2SAMZD1_DUAL_OUT_BM200H6P51CNODELVT", CellSourceType::Custom),
                    LogicBlock::AOI21 => ("AOI21SAMZD1BM200H6P51CNODELVT", CellSourceType::Custom),
                    LogicBlock::OAI21 => ("OAI21SAMZD1BM200H6P51CNODELVT", CellSourceType::Custom),
                    LogicBlock::AO21 => ("AO21SAMZD1BM200H6P51CNODELVT", CellSourceType::Custom),
                    LogicBlock::OA21 => ("OA21SAMZD1BM200H6P51CNODELVT", CellSourceType::Custom),
                    LogicBlock::IAOI21 => ("IAOI21SAMZD1BM200H6P51CNODELVT", CellSourceType::Custom),
                    LogicBlock::IOAI21 => ("IOAI21SAMZD1BM200H6P51CNODELVT", CellSourceType::Custom),
                    LogicBlock::AOI22 => ("AOI22SAMZD1BM200H6P51CNODELVT", CellSourceType::Custom),
                    LogicBlock::OAI22 => ("OAI22SAMZD1BM200H6P51CNODELVT", CellSourceType::Custom),
                    LogicBlock::AOAI211 => ("AOAI211SAMZD1BM200H6P51CNODELVT", CellSourceType::Custom),
                    LogicBlock::OAOI211 => ("OAOI211SAMZD1BM200H6P51CNODELVT", CellSourceType::Custom),
                    LogicBlock::AOA211 => ("AOA211MZD1BM200H6P51CNODELVT", CellSourceType::Custom),
                    LogicBlock::OAO211 => ("OAO211MZD1BM200H6P51CNODELVT", CellSourceType::Custom),
                    LogicBlock::Custom(..) => unimplemented!()
                }}
                Drive::D2 => { match logic_block {
                    LogicBlock::INV => ("INVMZD2BWP200H6P51CNODELVT", CellSourceType::Std),
                    LogicBlock::AOI21 => ("AOI21SAMZD2BM200H6P51CNODELVT", CellSourceType::Custom),
                    LogicBlock::OAI21 => ("OAI21SAMZD2BM200H6P51CNODELVT", CellSourceType::Custom),
                    _ => unimplemented!()
                }}
            }
        } else {
            return Self::parse_n4c_custom(logic_block, drive, custom_demand)
        };
        Self {
            name : name.to_string(),
            source_type,
            process : ProcessAndProject::N4C1340,
        }
        
    }
}


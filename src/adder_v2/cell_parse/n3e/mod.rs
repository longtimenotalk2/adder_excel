use crate::adder_v2::{adder::Cell, cell_info::Drive, cell_parse::{ReadCellName, ReadCellType}, logic::Logic};

impl Cell {
    pub fn parse_n3e_with_cell_type(&self) -> (ReadCellName, ReadCellType) {
        let (name, cell_type) = if self.info.is_default() {
            match self.info.drive {
                Drive::D1 => { match self.node.logic {
                    Logic::INV => ("INVD1BM156H3P48CPDELVT_1", ReadCellType::Std),
                    Logic::ND2 => ("ND2D1BM156H3P48CPDELVT_1", ReadCellType::Std),
                    Logic::NR2 => ("NR2D1BM156H3P48CPDELVT_1", ReadCellType::Std),
                    Logic::AN2 => ("AN2D1BM156H3P48CPDELVT_1", ReadCellType::Std),
                    Logic::OR2 => ("OR2D1BM156H3P48CPDELVT_1", ReadCellType::Std),
                    Logic::IND2 => ("IND2D1BM156H3P48CPDELVT_1", ReadCellType::Std),
                    Logic::INR2 => ("INR2D1BM156H3P48CPDELVT_1", ReadCellType::Std),
                    Logic::XOR2 => ("XOR2D1BM156H3P48CPDELVT", ReadCellType::Custom),
                    Logic::XNR2 => ("XNR2D1BM156H3P48CPDELVT", ReadCellType::Custom),
                    Logic::XOR2DOUT => ("XOR2D1_DUAL_OUT_BM156H3P48CPDELVT", ReadCellType::Custom),
                    Logic::XNR2DOUT => ("XNR2D1_DUAL_OUT_BM156H3P48CPDELVT", ReadCellType::Custom),
                    Logic::AOI21 => ("AOI21D1BM156H3P48CPDELVT", ReadCellType::Custom),
                    Logic::OAI21 => ("OAI21D1BM156H3P48CPDELVT", ReadCellType::Custom),
                    Logic::AO21 => ("AO21D1BM156H3P48CPDELVT", ReadCellType::Custom),
                    Logic::OA21 => ("OA21D1BM156H3P48CPDELVT", ReadCellType::Custom),
                    Logic::IAOI21 => ("IAOI21D1BM156H3P48CPDELVT", ReadCellType::Custom),
                    Logic::IOAI21 => ("IOAI21D1BM156H3P48CPDELVT", ReadCellType::Custom),
                    Logic::AOI22 => ("AOI22D1BM156H3P48CPDELVT", ReadCellType::Custom),
                    Logic::OAI22 => ("OAI22D1BM156H3P48CPDELVT", ReadCellType::Custom),
                    Logic::AOAI211 => ("AOAI211D1BM156H3P48CPDELVT", ReadCellType::Custom),
                    Logic::OAOI211 => ("OAOI211D1BM156H3P48CPDELVT", ReadCellType::Custom),
                    Logic::AOA211 => ("AOA211D1BM156H3P48CPDELVT", ReadCellType::Custom),
                    Logic::OAO211 => ("OAO211D1BM156H3P48CPDELVT", ReadCellType::Custom),
                }}
                Drive::D2 => { match self.node.logic {
                    Logic::INV => ("INVD2BM156H3P48CPDELVT_1", ReadCellType::Std),
                    Logic::AOI21 => ("AOI21D2BM156H3P48CPDELVT", ReadCellType::Custom),
                    Logic::OAI21 => ("OAI21D2BM156H3P48CPDELVT", ReadCellType::Custom),
                    Logic::ND2 => ("ND2D2BM156H3P48CPDELVT_1", ReadCellType::Std),
                    Logic::NR2 => ("NR2D2BM156H3P48CPDELVT_1", ReadCellType::Std),
                    Logic::IND2 => ("IND2D2BM156H3P48CPDELVT_1", ReadCellType::Std),
                    Logic::INR2 => ("INR2D2BM156H3P48CPDELVT_1", ReadCellType::Std),
                    Logic::AOI22 => ("AOI22D2BM156H3P48CPDELVT_1", ReadCellType::Std),
                    Logic::OAI22 => ("OAI22D2BM156H3P48CPDELVT_1", ReadCellType::Std),
                    _ => panic!("Default {:?} dont impl D2", self.node.logic)
                }}
            }
        } else {
            todo!()
        };

        (ReadCellName(name.to_string()), cell_type)
    }
}
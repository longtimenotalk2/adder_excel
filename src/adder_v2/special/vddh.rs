use crate::adder_v2::{cell::{cell_body::CellBody, cell_info::{CellInfo, Drive, SpecialInfo}}, cell_parse::{ReadCellName, ReadCellType}, logic::Logic};

impl CellInfo {
    pub fn is_power_vddh(&self) -> bool {
        self.special_infos.contains(&SpecialInfo("VDH".to_string()))
    }

    pub fn is_incr_cell(&self) -> bool {
        self.special_infos.contains(&SpecialInfo("VTPUL".to_string()))
    }
}

impl CellBody {
    pub fn parse_n3e_vdh(&self) -> (ReadCellName, ReadCellType) {
        let name = if self.contains_special("VTPUL") {
            match self.info.drive {
                Drive::D1 => {
                    match self.logic {
                        Logic::ND2 => "ND2D1BM156H3P48CPDELVT_1_P_ULVTLL_L2H_V03",
                        Logic::NR2 => "NR2D1BM156H3P48CPDELVT_1_P_ULVTLL_L2H_V03",
                        Logic::AOI21 => "AOI21D1BM156H3P48CPDELVT_P_ULVTLL_L2H_V03",
                        Logic::OAI21 => {
                            if self.contains_special("SP-BD") {
                                "OAI21D1BM156H3P48CPDELVT_P_ULVTLL_L2H_BD_V03"
                            } else {
                                "OAI21D1BM156H3P48CPDELVT_P_ULVTLL_L2H_V03"
                            }
                        },
                        _ => panic!("can not find {:?} D1 cell in N3E [VDH] [VTPUL]", self.logic),
                    }
                },
                _ => panic!("can not find {:?} {:?} cell in N3E [VDH] [VTPUL]", self.logic, self.info.drive),
            }
        } else {
            match self.info.drive {
                Drive::D1 => {
                    match self.logic {
                        Logic::AOI21 => "AOI21D1BM156H3P48CPDELVT_H2H_V03",
                        Logic::IAOI21 => "IAOI21D1BM156H3P48CPDELVT_H2H_V03",
                        Logic::INV => "INVD1BM156H3P48CPDELVT_1_H2H_V03",
                        Logic::IOAI21 => "IOAI21D1BM156H3P48CPDELVT_H2H_V02",
                        Logic::ND2 => "ND2D1BM156H3P48CPDELVT_1_H2H_V03",
                        Logic::NR2 => "NR2D1BM156H3P48CPDELVT_1_H2H_V02",
                        Logic::OAI21 => "OAI21D1BM156H3P48CPDELVT_H2H_V02",
                        _ => panic!("can not find {:?} D1 cell in N3E [VDH]", self.logic)
                    }
                }
                Drive::D2 => {
                    match self.logic {
                        Logic::AOI21 => "AOI21D2BM156H3P48CPDELVT_H2H_V03",
                        Logic::OAI21 => "OAI21D2BM156H3P48CPDELVT_H2H_V02",
                        Logic::INV => "INVD2BM156H3P48CPDELVT_1_H2H_V03",
                        _ => panic!("can not find {:?} D2 cell in N3E [VDH]", self.logic)
                    }
                }
                Drive::D3 => panic!("[VDH] {:?} dont impl D3", self.logic),
                Drive::D4 => match self.logic {
                    Logic::AOI21 => "AOI21D4BM156H3P48CPDELVT_H2H_V03",
                    Logic::OAI21 => "OAI21D4BM156H3P48CPDELVT_H2H_V02",
                    _ => panic!("can not find {:?} D4 cell in N3E [VDH]", self.logic)
                }
            }
        };
        (ReadCellName::new(name), ReadCellType::Lhw)
    }
}
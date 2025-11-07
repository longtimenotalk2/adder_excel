use std::collections::{BTreeMap, BTreeSet};

use crate::adder_v2::{adder::Adder, cell::{cell_body::CellBody, cell_info::{CellInfo, Drive, SpecialInfo}}, cell_parse::{Process, ReadCellName, ReadCellType}, logic::Logic, wire::Wire, Id, Port};

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

impl Adder {
    pub fn get_decr_info(&self) -> BTreeMap<Id, BTreeSet<Port>> {
        let mut ret = BTreeMap::new();

        let mut high_wires: Vec<(Id, Wire)> = vec![];

        for (i, cell) in self.cells.iter() {
            if cell.to_cell_body().info.is_power_vddh() {
                high_wires.push(cell.node.io.output_z.clone());
                if let Some(wire) = &cell.node.io.output_o1 {
                    high_wires.push(wire.clone());
                }
                if !cell.to_cell_body().info.is_incr_cell() {
                    for (port, input_wire) in &cell.node.io.input {
                        if !high_wires.contains(input_wire) {
                            println!(">>> warning !!! : for inst {} H2H cell, port {:?} with wire {} is not high!", cell.to_string(), port, input_wire.1.to_string());
                        }
                    }
                }
            } else {
                let mut high_port = BTreeSet::new();
                for (port, input_wire) in &cell.node.io.input {
                    if high_wires.contains(input_wire) {
                        high_port.insert(port.clone());
                    }
                }
                if high_port.len() > 0 {
                    ret.insert(i.clone(), high_port);
                }
            }
        }
        ret
    }

    pub fn get_decr_cell_new_name(&self, process : Process) -> BTreeMap<Id, String> {
        let decr_info = self.get_decr_info();
        let mut ret = BTreeMap::new();
        for (id, cell) in self.cells.iter() {
            if let Some(high_ports) = decr_info.get(id) {
                let mut added_txt = "_H2L".to_string();
                for port in high_ports {
                    added_txt.push_str(&format!("_{}", port.0));
                }
                ret.insert(id.clone(), format!("{}{}", cell.to_cell_body().parse(process).0.0, added_txt));
            }
        }
        ret
    }
}
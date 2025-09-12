use std::collections::{BTreeMap, BTreeSet};

use crate::{cell_parse::{ProcessAndProject, RealCell}, std::{adder::{Adder, CellFullInfoInAdder}, wire}};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConnectRelation {
    inputs : Vec<String>,
    outputs : Vec<String>,
    inst_name : String,
    cell_name : String,
}

impl ConnectRelation {
    pub fn to_string(&self) -> String {
        let input_txt = self.inputs.clone().into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let output_txt = self.outputs.clone().into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        format!("{};{};{};{}", self.inst_name, self.cell_name, input_txt, output_txt)
    }

    pub fn from_string(txt : &str) -> Self {
        let mut iter = txt.split(';');
        let inst_name = iter.next().unwrap().to_string();
        let cell_name = iter.next().unwrap().to_string();
        let inputs = iter.next().unwrap().split(',').map(|x| x.to_string()).collect::<Vec<_>>();
        let outputs = iter.next().unwrap().split(',').map(|x| x.to_string()).collect::<Vec<_>>();
        Self {
            inputs,
            outputs,
            inst_name,
            cell_name,
        }
    }

    pub fn from_cell_full_info(cell_info : &CellFullInfoInAdder, process : ProcessAndProject) -> Self {
        let inst_name = cell_info.inst_name();
        let map = &cell_info.logic_block_map;
        let abstract_cell = &cell_info.to_abstract_cell();
        let real_cell = RealCell::parse(process, abstract_cell);
        let cell_name = real_cell.name;

        let mut inputs = Vec::new();
        for (_port, wire) in &map.inputs {
            inputs.push(wire.to_string());
        }

        let mut outputs = Vec::new();
        let mut o1_wire = None;
        for (port, wire) in &map.outputs {
            if port.0.as_str() != "O1" {
                outputs.push(wire.to_string());
            } else {
                o1_wire = Some(wire);
            }
        }
        if let Some(o1_wire) = o1_wire {
            outputs.push(o1_wire.to_string());
        }

        Self {
            inputs,
            outputs,
            inst_name,
            cell_name,
        }
    }
}

impl Adder {
    pub fn get_connect_relation(&self, process_and_project : ProcessAndProject) -> Vec<ConnectRelation> {
        let mut ret = vec![];
        for cell_info in &self.cells {
            ret.push(ConnectRelation::from_cell_full_info(&cell_info, process_and_project));
        }
        ret
    }

    pub fn save_connect_relation(&self, process_and_project : ProcessAndProject) {
        let mut txt = String::new();
        for connect_relation in self.get_connect_relation(process_and_project) {
            txt += &connect_relation.to_string();
            txt += "\n";
        }
        use std::fs::File;
        use std::io::prelude::*;
        let content = "This is the content to write to the file.";
        // 创建一个新文件，如果文件已存在，则覆盖
        let mut file = File::create("output.txt").unwrap();
        // 将字符串写入文件
        let _ = file.write_all(txt.as_bytes());
    }
}
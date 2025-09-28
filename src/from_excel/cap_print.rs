use std::collections::BTreeMap;

use crate::{from_excel::{ExcelData, LayerType}, std::logic_block::Port};

impl ExcelData {
    pub fn cap_check(&self, caps : &Vec<BTreeMap<Port, i32>>) {
        for (i, cap_dict) in caps.iter().enumerate() {
            
            if i < self.excel_layout_positions.len() {
                let mut cap_vec = vec![];
                if cap_dict.len() == 1 {
                    cap_vec.push(cap_dict.values().next().unwrap().clone());
                } else if cap_dict.len() == 2 {
                    // O1 放在前面
                    cap_vec.push(cap_dict.get(&Port("O1".to_string())).unwrap().clone());
                    let keys = cap_dict.keys();
                    for key in keys {
                        if key != &Port("O1".to_string()) {
                            cap_vec.push(cap_dict.get(key).unwrap().clone());
                        }
                    }
                }
                let pos = self.excel_layout_positions[i];
                let cap_excel = self.excel_cap_data.get(&pos);
                if let Some(cap_excel) = cap_excel {
                    if cap_vec != *cap_excel {
                        println!("error : CAP mismatch at excel {pos:?} : calc is {cap_vec:?}, excel is {cap_excel:?}");
                    }
                } else {
                    println!("warning : NO VALUE at excel {pos:?} : calc is {cap_vec:?}" );
                }
            }
            
        }
        println!("cap ckeck end!!")
    }
    pub fn cap_print(&self, caps : &Vec<BTreeMap<Port, i32>>) {
        let mut cap_layout: BTreeMap<(usize, usize), Vec<i32>> = BTreeMap::new();
        for (i, pos) in self.excel_layout_positions.iter().enumerate() {
            let cap_dict = caps[i].clone();
            let mut cap_vec = vec![];
            if cap_dict.len() == 1 {
                cap_vec.push(cap_dict.values().next().unwrap().clone());
            } else if cap_dict.len() == 2 {
                // O1 放在前面
                cap_vec.push(cap_dict.get(&Port("O1".to_string())).unwrap().clone());
                let keys = cap_dict.keys();
                for key in keys {
                    if key != &Port("O1".to_string()) {
                        cap_vec.push(cap_dict.get(key).unwrap().clone());
                    }
                }
            }
            cap_layout.insert((pos.0, pos.1), cap_vec);
        }
        let mut cap_layout_vec: Vec<Vec<Vec<i32>>> = vec![vec![vec![]; self.bits]; self.excel_row_layer.len()];
        for (pos, cap_vec) in cap_layout.iter() {
            cap_layout_vec[pos.0][pos.1] = cap_vec.clone();
        }

        let mut s = String::new();
        for (i, row_info) in cap_layout_vec.iter().enumerate() {
            let layer = self.excel_row_layer[i];
            s += &format!("{}\t", layer);
            let layer_type = &self.excel_row_layer_type[i];
            let layer_type_string = match layer_type {
                LayerType::Cri => "cri",
                LayerType::Uncri => "uncri",
            };
            s += &format!("{}\t", layer_type_string);
            s += &format!("cap");

            for cap_vec in row_info.iter().rev() {
                s += "\t";
                if cap_vec.len() > 0 {
                    let cap_string = cap_vec.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
                    s += &cap_string;
                }
            }
            s += "\n";
        }

        use std::fs::File;
        use std::io::prelude::*;
        let content = "This is the content to write to the file.";
        // 创建一个新文件，如果文件已存在，则覆盖
        let mut file = File::create("output.txt").unwrap();
        // 将字符串写入文件
        let _ = file.write_all(s.as_bytes());

    }
}
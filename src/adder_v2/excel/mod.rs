pub mod excel_to_datalist;
use std::collections::BTreeMap;

use crate::adder_v2::Id;

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct ExcelKey {
    pub multi_line_id : Id,
}

#[derive(Debug, Clone, Default)]
pub struct ExcelMultiLineData {
    pub wire_line : Vec<String>,
    pub layer : i32,
    pub name : String,
    pub code_line : Vec<String>,
    pub cap_line : Vec<String>,
}


#[derive(Debug, Clone)]
pub struct ExcelFrame {
    pub multi_lines : BTreeMap<ExcelKey, ExcelMultiLineData>,
    pub bits : usize,
}

impl ExcelFrame {
    pub fn load(path : &str) -> Self {
        let file = std::fs::File::open(path).expect(&format!("file {path} not exist"));
        let reader = std::io::BufReader::new(file);
        let lines : Vec<String> = std::io::BufRead::lines(reader).map(|l| l.unwrap()).collect();

        let mut id = 0;
        let mut multi_lines = BTreeMap::new();
        let mut bits = None;

        for line in lines {
            let items = line.split("\t").map(|s| s.trim()).collect::<Vec<&str>>();
            if line.starts_with("LY") {
                bits = Some(items[3].parse::<usize>().unwrap() + 1);
            } else {
                let layer = items[0].parse::<i32>().unwrap();
                let name = items[1].to_string();
                let data_iter = items[3..(3+bits.unwrap())].iter().rev();
                let mut data_line = vec![];
                for data in data_iter {
                    data_line.push(data.to_string());
                }
                
                // 确保wire层在最前面
                match items[2] {
                    "wire" => {
                        id += 1;
                        let mut dataset = ExcelMultiLineData::default();
                        dataset.layer = layer;
                        dataset.name = name;
                        dataset.wire_line = data_line;
                        multi_lines.insert(ExcelKey { multi_line_id : id }, dataset);
                    }
                    _ => {
                        let dataset = multi_lines.get_mut(&ExcelKey { multi_line_id : id }).unwrap();
                        match items[2] {
                            "code" => dataset.code_line = data_line,
                            "cap" => dataset.cap_line = data_line,
                            _ => panic!("invalid type {} in excel", items[2]),
                        }
                    }
                }
                
            }
        }

        Self {
            multi_lines,
            bits : bits.unwrap(),
        }
        
    }
}


use crate::from_excel::{ExcelData, LayerType, Node};

impl ExcelData {
    pub fn load(path : &str) -> Self {
        let mut nodes : Vec<Node> = vec![];
        
        let file = std::fs::File::open(path).expect("file not found");
        let reader = std::io::BufReader::new(file);
        let lines : Vec<String> = std::io::BufRead::lines(reader).map(|l| l.unwrap()).collect();

        let items : Vec<_> = lines[0].split("\t").collect();
        let bits = items[3].parse::<usize>().unwrap() + 1;

        let mut wire_layer_to_be_match: Option<String> = None;

        for line in &lines[1..] {
            let items : Vec<_> = line.split("\t").collect();
            if items[2].trim() == "wire" {
                assert!(wire_layer_to_be_match.is_none());
                wire_layer_to_be_match = Some(line.to_string());
            } else if items[2].trim() == "code" {
                let items_pre : Vec<_> = wire_layer_to_be_match.as_ref().unwrap().split("\t").collect();
                assert!(items_pre[0] == items[0]);
                assert!(items_pre[1] == items[1]);
                // 开始生成
                let wires : Vec<_> = items_pre[3..].iter().rev().map(|s| s.to_string()).collect();
                let codes : Vec<_> = items[3..].iter().rev().map(|s| s.to_string()).collect();
                for index in 0..bits {
                    let wire = wires[index].trim();
                    let code = codes[index].trim();
                    if wire != "" {
                        nodes.push(Node {
                            index,
                            wire_txt: wire.to_string(),
                            code_txt: code.to_string(),
                            layer : items[0].parse().unwrap(),
                            layer_type : match items[1].trim(){
                                "cri" => LayerType::Cri,
                                "uncri" => LayerType::Uncri,
                                _ => unimplemented!()
                            }
                        });
                    }
                }
                wire_layer_to_be_match = None;
            }
        }

        Self {
            bits,
            nodes,
        }
    }
}

#[test]
fn test_load() {
    dbg!(ExcelData::load("src/from_excel/data/uf31.txt"));
}
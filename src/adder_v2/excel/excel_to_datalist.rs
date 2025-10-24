use std::collections::{BTreeMap, BTreeSet};

use crate::adder_v2::{adder::{CellInfo, Drive}, excel::ExcelFrame, node::{FlagPChain, NodeHint}, wire::{Flag, FlagP, Wire, WireFloat}, Id};

// 每个multi line的每个index只有对多一个cell。保证顺序是从上行到下行，每一行从index小到index大
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct ExcelDataListKey {
    multi_line_id : Id,
    index : usize,
}

#[derive(Debug, Clone)]
pub struct ExcelDataList<T> {
    pub data : BTreeMap<ExcelDataListKey, T>,
    pub bits : usize,
}

#[derive(Debug, Clone)]
struct ExcelCode {
    single_chars : BTreeSet<char>,
    flower_braket : BTreeMap<char, String>,
}

impl ExcelCode {
    fn from_str(s : &str) -> Self {
        let mut single_chars = BTreeSet::new();
        let mut flower_braket = BTreeMap::new();

        let chars = s.chars().collect::<Vec<_>>();
        let mut char_now: Option<char> = None;
        let mut in_flower = String::new();
        let mut now_in_flower = false;
        for char in chars {
            if char == '{' {
                now_in_flower = true;
            } else if char == '}' {
                now_in_flower = false;
                flower_braket.insert(char_now.unwrap(), in_flower);
                in_flower = String::new();
                char_now = None;
            } else {
                if now_in_flower {
                    in_flower.push(char);
                } else {
                    if let Some(c) = char_now {
                        single_chars.insert(c);
                    }
                    char_now = Some(char);
                }
            }
        }

        if let Some(c) = char_now {
            single_chars.insert(c);
        }

        Self {
            single_chars,
            flower_braket,
        }
    }
}

#[test]
fn test_excel_code() {
    let code = "DL{hph}N";
    let excel_code = ExcelCode::from_str(code);
    println!("{:?}", excel_code);
}

impl<T> ExcelDataList<T> {
    pub fn new_with_bits(bits : usize) -> Self {
        Self {
            data : BTreeMap::new(),
            bits,
        }
    }
}

//                                      cap
impl ExcelDataList<(NodeHint, CellInfo, Option<Vec<i32>>)> {
    pub fn from_excel_frame(frame : &ExcelFrame) -> Self {
        let mut dataset = BTreeMap::new();
        for (key, data) in frame.multi_lines.iter() {
            let  multi_line_id = key.id;
            for index_excel in 0..frame.bits {
                let mut wire_string = data.wire_line[index_excel].clone();
                if wire_string.len() > 0 {
                    let mut index = index_excel;
                    while wire_string.starts_with("<") {
                        wire_string = wire_string[1..].to_string();
                        index += 1;
                    }
                    let code = ExcelCode::from_str(data.code_line.get(index_excel).unwrap_or(&"".to_string()));
                    let caps = if let Some(caps_string) = data.cap_line.get(index_excel) {
                        if caps_string.len() > 0 {
                            Some(caps_string.split(",").into_iter().map(|a| a.parse::<i32>().expect(
                                &format!("can not parse caps {caps_string}"))
                            ).collect::<Vec<_>>())
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    let mut node_hint = NodeHint::default();
                    node_hint.given_out_index = index;
                    node_hint.given_out_len = 1;
                    let mut cell_info = CellInfo::default();
                    match wire_string.as_str() {
                        "nq~" => node_hint.is_start_xnr_dout = true,
                        "nq"  => node_hint.is_start_xnr = true,
                        "q~"  => node_hint.is_start_xor_dout = true,
                        "q"   => node_hint.is_start_xor = true,
                        _ => {
                            let wire_float = WireFloat::from_str(&wire_string);
                            let wire = Wire::from_wire_float(wire_float, index);
                            node_hint.given_out_flag_p = Some(wire.to_flag_p());
                            if code.single_chars.contains(&'I') {
                                node_hint.is_simple_inv = true;
                            }
                            if code.single_chars.contains(&'A') {
                                node_hint.is_start = true;
                            }
                            if code.single_chars.contains(&'N') {
                                node_hint.is_out_addition_inv = true;
                            }
                            if let Some(chain_str) = code.flower_braket.get(&'L') {
                                let mut is_neg_now = false;
                                let mut flagplist = Vec::new();
                                for char in chain_str.chars() {
                                    if char == '~' {
                                        is_neg_now = true;
                                    } else {
                                        let flag = Flag::from_str(&char.to_string());
                                        let flagp = FlagP::new(flag, is_neg_now);
                                        flagplist.push(flagp);
                                        is_neg_now = false;
                                    }
                                }
                                node_hint.given_flag_p_chain = Some(FlagPChain(flagplist));
                            }

                            if code.single_chars.contains(&'D') {
                                cell_info.drive = Drive::D2;
                            }
                        }
                    }
                    let excel_data_list_key = ExcelDataListKey {
                        multi_line_id,
                        index : index_excel,
                    };
                    dataset.insert(excel_data_list_key, (node_hint, cell_info, caps));
                }
                
            }
        }
        Self {
            data: dataset,
            bits : frame.bits,
        }
    }
}
use std::collections::{BTreeMap, BTreeSet};

use crate::std::{adder::{CellHinter, Drive}, node_create::LogiBlockHint, wire::{Flag, Wire}};


impl CellHinter {
    pub fn new(wire_txt : &str, code_txt : &str, index : usize) -> Self {
        let code = Code::from_code_txt(code_txt);
        let (index, wire_txt) = if wire_txt.starts_with('<') {
            (index + 1, &wire_txt[1..])
        } else {
            (index, wire_txt)
        };
        let wire = Wire::from_str_index(&wire_txt.replace("~", ""), index);
        let logic_block_hints =  match wire_txt {
            "q~" => vec![LogiBlockHint::XORDOUT(code.char_single.contains(&'E'))],
            "nq~" => vec![LogiBlockHint::XNRDOUT(code.char_single.contains(&'E'))],
            _ => {
                if code.char_single.contains(&'I') {
                    vec![LogiBlockHint::INV]
                } else if code.char_single.contains(&'A') {
                    vec![LogiBlockHint::OnlyFromAB]
                } else {
                    let is_out_inv = code.char_single.contains(&'N');
                    if let Some(lcode) = code.char_string.get(&'L') {
                        let normal_code = NormalHint::new(lcode);
                        vec![LogiBlockHint::Normal {
                            flags : normal_code.flags,
                            custom_input_invs : normal_code.custom_input_invs,
                            custom_input_lens : normal_code.custom_input_lens,
                            is_out_inv,
                        }]
                    } else {
                        match wire.flag {
                            Flag::G => {
                                vec![
                                    vec![Flag::G, Flag::P, Flag::G],
                                    vec![Flag::G, Flag::Q, Flag::G],
                                ].into_iter().map(|flags| {
                                    LogiBlockHint::Normal {
                                        flags,
                                        custom_input_invs : BTreeSet::new(),
                                        custom_input_lens : BTreeMap::new(),
                                        is_out_inv : false,
                                    }
                                }).collect()
                            }
                            Flag::H => {
                                vec![
                                    vec![Flag::H, Flag::P, Flag::H],
                                ].into_iter().map(|flags| {
                                    LogiBlockHint::Normal {
                                        flags,
                                        custom_input_invs : BTreeSet::new(),
                                        custom_input_lens : BTreeMap::new(),
                                        is_out_inv : false,
                                    }
                                }).collect()
                            }
                            Flag::P => {
                                vec![
                                    vec![Flag::P, Flag::P],
                                    vec![Flag::Q, Flag::P],
                                ].into_iter().map(|flags| {
                                    LogiBlockHint::Normal {
                                        flags,
                                        custom_input_invs : BTreeSet::new(),
                                        custom_input_lens : BTreeMap::new(),
                                        is_out_inv : false,
                                    }
                                }).collect()
                            }
                            Flag::Q => {
                                vec![
                                    vec![Flag::P, Flag::Q],
                                    vec![Flag::Q, Flag::Q],
                                ].into_iter().map(|flags| {
                                    LogiBlockHint::Normal {
                                        flags,
                                        custom_input_invs : BTreeSet::new(),
                                        custom_input_lens : BTreeMap::new(),
                                        is_out_inv : false,
                                    }
                                }).collect()
                            }
                            _ => unimplemented!()
                        }
                    }
                }
            },
        };

        let drive = if code.char_single.contains(&'D') {
            Drive::D2
        } else {
            Drive::D1
        };

        Self {
            logic_block_hints : logic_block_hints,
            drive,
            custom_demand : vec![],
        }
    }
}

struct NormalHint {
    flags : Vec<Flag>, // 从index大到index小的顺序的输入的flag
    custom_input_invs : BTreeSet<usize>, // 默认输入是同极性的，这里列的是输入极性相反的地址
    custom_input_lens : BTreeMap<usize, usize>, // 如果强制指定某个输入的长度，在这里指定
} 

impl NormalHint {
    fn new(txt : &str) -> Self {
        let mut remained : String = txt.to_string();

        let mut flags : Vec<Flag> = Vec::new();
        let mut custom_input_invs : BTreeSet<usize> = BTreeSet::new();
        let custom_input_lens : BTreeMap<usize, usize> = BTreeMap::new();

        while remained.len() > 0 {
            let is_inv = if remained.starts_with("~") {
                remained = remained[1..].to_string();
                true
            } else {
                false
            };
            let flag = Flag::from_str(remained.remove(0).to_string().as_str());

            flags.push(flag);
            if is_inv {
                custom_input_invs.insert(flags.len()-1);
            }
        }

        NormalHint {
            flags,
            custom_input_invs,
            custom_input_lens,
        }
    }
}

struct Code {
    char_single : BTreeSet<char>,
    char_string : BTreeMap<char, String>,
    custom : String,
}

impl Code {
    fn from_code_txt(code : &str) -> Code {
        let mut remained : String = code.to_string();
        let mut char_single : BTreeSet<char> = BTreeSet::new();
        let mut char_string : BTreeMap<char, String> = BTreeMap::new();
        while remained.len() > 0 {
            if remained.len() > 1 && &remained[1..2] == "{" {
                let kat_end = remained.find("}").unwrap();
                char_string.insert(remained.chars().nth(0).unwrap(), remained[2..kat_end].to_string());
                remained = remained[kat_end+1..].to_string();
            } else {
                char_single.insert(remained.chars().nth(0).unwrap());
                remained = remained[1..].to_string();
            }
        }

        Code {
            char_single,
            char_string,
            custom : "".to_string(),
        }
    }
}

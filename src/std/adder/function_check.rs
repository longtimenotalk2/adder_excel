use std::collections::BTreeMap;

use colorful::{Color, Colorful};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha12Rng;

use crate::{from_excel::ExcelData, std::{adder::{Adder, CellFullInfoInAdder}, wire::Wire}};

impl CellFullInfoInAdder {
    fn execute_with_signals(&self, signals : &mut BTreeMap<Wire, bool>) {
        self.logic_block_map.execute_with_signals(signals);
    }
}

pub struct FunctionError {
    a : Vec<bool>,
    b : Vec<bool>,
    s_out : Vec<bool>,
    actual_a : Vec<bool>,
    actual_b : Vec<bool>,
    actual_s : Vec<bool>,
    actual_s_out : Vec<bool>,
    signals : BTreeMap<Wire, bool>,
}

impl FunctionError {
    pub fn to_string(&self) -> String {
        let mut txt = String::new();
        for (wire, value) in self.signals.iter() {
            txt += &format!("{:>8?} : {}\n", wire, if *value {"1"} else {"0"});
        }
        txt += &format!("{:>16} : {}\n", "a", bool_list_show(&self.a));
        txt += &format!("{:>16} : {}\n", "b", bool_list_show(&self.b));
        txt += &format!("{:>16} : {}\n", "s_out", bool_list_match_with_color(&self.s_out, &if self.a[0] == self.actual_a[0] {
            self.actual_s.to_vec()
        } else {
            bool_list_inv(&self.actual_s)
        }));
        txt += &format!("{:>16} : {}\n", "actual_a", bool_list_show(&self.actual_a));
        txt += &format!("{:>16} : {}\n", "actual_b", bool_list_show(&self.actual_b));
        txt += &format!("{:>16} : {}\n", "actual_s", bool_list_show(&self.actual_s));
        txt += &format!("{:>16} : {}\n", "actual_s_out", bool_list_match_with_color(&self.actual_s_out, &self.actual_s));
        txt
    }
}

fn bool_list_inv(v : &[bool]) -> Vec<bool> {
    v.iter().map(|x| !x).collect()
}

fn bool_list_show(v : &[bool]) -> String {
    let mut s = String::new();
    for (i, b) in v.iter().enumerate() {
        if i > 0 && i % 4 == 0 {
            s = format!("_{s}");
        }
        s = format!("{}{s}", if *b {"1"} else {"0"});
    }
    s
}

fn bool_list_match_with_color(v : &[bool], v_check : &[bool]) -> String {
    let mut s = String::new();
    for (i, b) in v.iter().enumerate() {
        if i > 0 && i % 4 == 0 {
            s = format!("_{s}");
        }
        let is_correct = v[i] == v_check[i];
        let value = if *b {"1"} else {"0"};
        let value_colored = if is_correct {value.color(Color::Green)} else {value.color(Color::Red)};
        s = format!("{}{s}", value_colored);
    }
    s
}

fn bool_list_add(v1 : &[bool], v2 : &[bool]) -> Vec<bool> {
    assert_eq!(v1.len(), v2.len());

    let mut result = Vec::with_capacity(v1.len());
    let mut c = false;
    for (&a, &b) in v1.iter().zip(v2.iter()) {
        result.push(a ^ b ^ c);
        c = (a && b) || (a && c) || (b && c);
    }
    result
}

impl Adder {
    pub fn execute(&self, a : Vec<bool>, b : Vec<bool>) -> (Vec<bool>, BTreeMap<Wire, bool>) {
        assert_eq!(self.bits, a.len());
        assert_eq!(self.bits, b.len());

        let mut signals : BTreeMap<Wire, bool> = BTreeMap::new();
        for i in 0..self.bits {
            signals.insert(Wire::from_str(&format!("a{}", i)), a[i]);
            signals.insert(Wire::from_str(&format!("b{}", i)), b[i]);
        }

        for cell in self.cells.iter() {
            cell.execute_with_signals(&mut signals)
        }

        let mut s = vec![];
        for i in 0..self.bits {
            // let wire = if self.output_is_neg {Wire::from_str(&format!("ns{}", i))} else {Wire::from_str(&format!("s{}", i))};
            let wire = Wire::from_str(&format!("s{}", i));
            s.push(*signals.get(&wire).expect(&format!("can not found {wire:?}, wire list : {:?}", signals)));
        }

        (s, signals)
    }
    pub fn function_check_main(&self, a : Vec<bool>, b : Vec<bool>) -> Result<(), FunctionError> {
        assert_eq!(self.bits, a.len());
        assert_eq!(self.bits, b.len());

        let actual_a = if self.input_is_neg {bool_list_inv(&a)} else {a.clone()};
        let actual_b = if self.input_is_neg {bool_list_inv(&b)} else {b.clone()};
        let actual_s = bool_list_add(&actual_a, &actual_b);

        let (s_out, signals) = self.execute(a.clone(), b.clone());
        let actual_s_out = if self.output_is_neg {bool_list_inv(&s_out)} else {s_out.clone()};

        if actual_s == actual_s_out {
            Ok(())
        } else {
            Err(FunctionError {
                a,
                b,
                actual_a,
                actual_b,
                actual_s,
                s_out,
                actual_s_out,
                signals
            })
        }
    }

    pub fn function_check_given_patterns(&self, patterns : Vec<(Vec<bool>, Vec<bool>)>)  {
        for (a, b) in patterns {
            if let Err(error) = self.function_check_main(a, b) {
                println!("{}", error.to_string());
                println!("test fail!");
                return;
            }
        }
        println!("test pass");
    }

    pub fn function_check_random(&self, count : usize, seed : u64)  {
        let mut rng = ChaCha12Rng::seed_from_u64(seed);
        for i in 0..count {
            let a = (0..self.bits).map(|_| rng.random()).collect();
            let b = (0..self.bits).map(|_| rng.random()).collect();
            if let Err(error) = self.function_check_main(a, b) {
                println!("{}", error.to_string());
                println!("test {} fail!", format!("{i}").color(Color::Red));
                return;
            }
        }
        println!("{} test pass", format!("{count}").color(Color::Green));
    }
}

fn u31_to_vec(v : u32) -> Vec<bool> {
    let mut result = vec![];
    for i in 0..31 {
        result.push((v >> i) & 1 == 1);
    }
    result
}

#[test]
fn test_function() {
    let excel_data = ExcelData::load("src/from_excel/data/uf31.txt");
    let adder = excel_data.create(31, false, true);
    // dbg!(&adder);
    adder.function_check_random(10, 0);
    
    // adder.function_check_given_patterns(vec![
    //     (u31_to_vec(0), u31_to_vec(0)),
    // ])
}
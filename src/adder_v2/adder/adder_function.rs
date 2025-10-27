use std::collections::BTreeMap;

use colorful::{Color, Colorful};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha12Rng;

use crate::adder_v2::{adder::Adder, wire::Wire, Id};

impl Adder {
    pub fn execute(&self, a : Vec<bool>, b : Vec<bool>) -> (Vec<bool>, BTreeMap<(Id, Wire), bool>) {
        assert_eq!(self.bits, a.len());
        assert_eq!(self.bits, b.len());

        let mut wire_id : Id = 0;

        let mut value_tabel : BTreeMap<(Id, Wire), bool> = BTreeMap::new();

        for i in 0..self.bits {
            value_tabel.insert((wire_id, Wire::from_str(&format!("a{i}"))), a[i]);
            wire_id += 1;
            value_tabel.insert((wire_id, Wire::from_str(&format!("b{i}"))), b[i]);
            wire_id += 1;
        }

        for (_, cell) in self.cells.iter() {
            cell.node.calc_with_list(&mut value_tabel);
        }


        let mut s = vec![];
        for i in 0..self.bits {
            let mut is_found = false;
            for ((_, wire), value) in value_tabel.iter().rev() {
                if wire.to_string() == format!("s{i}") {
                    s.push(*value);
                    is_found = true;
                    break;
                }
            }
            if !is_found {
                let mut txt = String::new();
                for ((id, wire), value) in value_tabel.iter() {
                    txt += &format!("> {:03} : {} = {}\n", id,wire.to_string(), if *value {"1"} else {"0"});
                }
                println!("{}", txt);
                panic!("can not found s{i}");
            }
        }

        (s, value_tabel)
    }

    pub fn check_function_given(&self, circuit_a : Vec<bool>, circuit_b : Vec<bool>) -> Result<FunctionError, FunctionError> {
        let math_a = if self.input_is_neg {bool_list_inv(&circuit_a)} else {circuit_a.clone()};
        let math_b = if self.input_is_neg {bool_list_inv(&circuit_b)} else {circuit_b.clone()};
        let math_s = bool_list_add(&math_a, &math_b);

        let (circuit_s, value_table) = self.execute(circuit_a.clone(), circuit_b.clone());

        let circuit_s_golden = if self.output_is_neg {bool_list_inv(&math_s)} else {math_s.clone()};

        if circuit_s == circuit_s_golden {
            Ok(FunctionError {
                math_a,
                math_b,
                math_s,
                circuit_a,
                circuit_b,
                circuit_s,
                circuit_s_golden,
                value_table,
            })
        } else {
            Err(FunctionError {
                math_a,
                math_b,
                math_s,
                circuit_a,
                circuit_b,
                circuit_s,
                circuit_s_golden,
                value_table,
            })
        }
    }

    pub fn check_function_random(&self, n : usize) {
        print!(">>> start check adder_{} function with {} random nunbers ...  ", self.polar_name_lowercase(), format!("{n}").color(Color::Yellow));
        let mut rng = ChaCha12Rng::seed_from_u64(0);
        let mut circuit_a = vec![];
        let mut circuit_b = vec![];
        for i in 0..n {
            circuit_a.clear();
            circuit_b.clear();
            
            for _ in 0..self.bits {
                circuit_a.push(rng.random());
                circuit_b.push(rng.random());
            }
            if let Err(err) = self.check_function_given(circuit_a.clone(), circuit_b.clone()) {
                println!("{}", "error !".to_string().color(Color::Red));
                println!("{}", self.cells_to_string());
                println!("{}", err.to_string());
                println!("Error at test {i}/{n} : ");
                panic!();
            }
        }
        println!("{}", "pass !".to_string().color(Color::Green));
        let example = self.check_function_given(circuit_a.clone(), circuit_b.clone()).unwrap();
        println!("{}", example.to_string_wo_values());
    }
}

#[derive(Debug, Clone)]
pub struct FunctionError {
    math_a : Vec<bool>,
    math_b : Vec<bool>,
    math_s : Vec<bool>,
    circuit_a : Vec<bool>,
    circuit_b : Vec<bool>,
    circuit_s : Vec<bool>,
    circuit_s_golden : Vec<bool>,
    value_table : BTreeMap<(Id, Wire), bool>,
}

impl FunctionError {
    pub fn to_string(&self) -> String {
        let mut txt = String::new();
        txt += "values ======================================================\n";
        for ((id, wire), value) in self.value_table.iter() {
            txt += &format!("> {:03} : {} = {}\n", id,wire.to_string(), if *value {"1"} else {"0"});
        }

        txt += "math ========================================================\n";
        txt += &format!("{:>16} : {}\n", "math_a", bool_list_show(&self.math_a));
        txt += &format!("{:>16} : {}\n", "math_b", bool_list_show(&self.math_b));
        txt += &format!("{:>16} : {}\n", "math_s", bool_list_show(&self.math_s));

        txt += "circuit =====================================================\n";
        txt += &format!("{:>16} : {}\n", "circuit_a", bool_list_show(&self.circuit_a));
        txt += &format!("{:>16} : {}\n", "circuit_b", bool_list_show(&self.circuit_b));
        txt += &format!("{:>16} : {}\n", "circuit_s_golden", bool_list_show(&self.circuit_s_golden));
        txt += &format!("{:>16} : {}\n", "circuit_s_actual", bool_list_match_with_color(&self.circuit_s, &self.circuit_s_golden));
        txt
    }

    pub fn to_string_wo_values(&self) -> String {
        let mut txt = String::new();

        txt += "math ========================================================\n";
        txt += &format!("{:>16} : {}\n", "math_a", bool_list_show(&self.math_a));
        txt += &format!("{:>16} : {}\n", "math_b", bool_list_show(&self.math_b));
        txt += &format!("{:>16} : {}\n", "math_s", bool_list_show(&self.math_s));

        txt += "circuit =====================================================\n";
        txt += &format!("{:>16} : {}\n", "circuit_a", bool_list_show(&self.circuit_a));
        txt += &format!("{:>16} : {}\n", "circuit_b", bool_list_show(&self.circuit_b));
        txt += &format!("{:>16} : {}\n", "circuit_s_golden", bool_list_show(&self.circuit_s_golden));
        txt += &format!("{:>16} : {}\n", "circuit_s_actual", bool_list_match_with_color(&self.circuit_s, &self.circuit_s_golden));
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
use std::fmt::Debug;

use crate::std::wire::{Flag, Wire};

impl Flag {
    pub fn from_str(s: &str) -> Self {
        match s {
            "g" => Flag::G,
            "p" => Flag::P,
            "q" => Flag::Q,
            "h" => Flag::H,
            "s" => Flag::S,
            "a" => Flag::A,
            "b" => Flag::B,
            _ => panic!("can not parse Flag with {s}"),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Flag::G => "g",
            Flag::P => "p",
            Flag::Q => "q",
            Flag::H => "h",
            Flag::S => "s",
            Flag::A => "a",
            Flag::B => "b",
        }
    }
}

impl Debug for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Wire {
    pub fn new(flag: Flag, is_neg: bool, index: usize, len: usize) -> Self {
        Wire {
            flag,
            is_neg,
            index,
            len,
        }
    }

    pub fn rev(&self) -> Self {
        let mut ret = self.clone();
        ret.is_neg = !ret.is_neg;
        ret
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        if self.is_neg && self.flag != Flag::S {
            ret.push('n');
        }
        ret.push_str(self.flag.to_str());
        if self.len > 1 {
            if self.index + 1 < self.len {
                panic!("bad index and len combination for flag : {:?}, index : {}, len : {}", self.flag, self.index, self.len);
            }
            ret.push_str(&format!("{index}_{len}", index = self.index, len = self.index + 1 - self.len));
        } else {
            ret.push_str(&format!("{}", self.index));
        }

        ret
    }

    pub fn from_str(s : &str) -> Self {
        let (is_neg, mut remained) = if s.starts_with("n") {
            (true, s[1..].to_string())
        } else {
            (false, s.to_string())
        };
        
        let flag = Flag::from_str(&remained[0..1]);
        remained = remained[1..].to_string();

        

        let (index, len) = if remained.contains("_") {
            let tokens = remained.split("_").collect::<Vec<&str>>();
            let index = tokens[0].parse::<usize>().unwrap();
            let index_start = tokens[1].parse::<usize>().unwrap();
            let len = index + 1 - index_start;
            (index, len)
        } else {
            let index = remained.parse::<usize>().unwrap();
            (index, 1)
        };

        Wire {
            flag,
            is_neg,
            index,
            len,
        }
    }

    pub fn from_str_index_len(s : &str, index : usize, len : usize) -> Self {
        let (is_neg, mut remained) = if s.starts_with("n") {
            (true, s[1..].to_string())
        } else {
            (false, s.to_string())
        };
        
        let flag = Flag::from_str(&remained[0..1]);
        remained = remained[1..].to_string();



        Wire {
            flag,
            is_neg,
            index,
            len,
        }
    }

    pub fn from_str_index(s : &str, index : usize) -> Self {
        let (is_neg, mut remained) = if s.starts_with("n") {
            (true, s[1..].to_string())
        } else {
            (false, s.to_string())
        };
        
        let flag = Flag::from_str(&remained[0..1]);
        remained = remained[1..].to_string();

        let len =if remained.len() == 0 {
            1
        } else {
            remained.parse::<usize>().expect(&format!("wire {s} can not parse from_str_index"))
        };

         

        Wire {
            flag,
            is_neg,
            index,
            len,
        }
    }

    pub fn if_rev(self, is_rev: bool) -> Self {
        if is_rev {
            self.rev()
        } else {
            self
        }
    }
}
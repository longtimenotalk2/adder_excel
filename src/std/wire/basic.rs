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
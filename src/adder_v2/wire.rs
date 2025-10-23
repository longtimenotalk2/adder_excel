pub mod wire_list;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Flag {
    A,
    B,
    S,
    G,
    P,
    Q,
    H,
}

impl Flag {
    pub fn to_str(&self) -> &str {
        match self {
            Flag::A => "a",
            Flag::B => "b",
            Flag::S => "s",
            Flag::G => "g",
            Flag::P => "p",
            Flag::Q => "q",
            Flag::H => "h",
        }
    }

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlagExtend {
    pub flag: Flag,
    pub is_neg: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireFloat {
    pub flag: Flag,
    pub is_neg: bool,
    pub len: usize,
}

impl WireFloat {
    pub fn from_str(s : &str) -> Self {
        let (is_neg, mut remained) = if s.starts_with("n") {
            (true, s[1..].to_string())
        } else {
            (false, s.to_string())
        };
        
        let flag = Flag::from_str(&remained[0..1]);
        remained = remained[1..].to_string();

        let len = if remained.len() == 0 {
            1
        } else {
            remained.parse::<usize>().unwrap()
        };

        Self {
            flag,
            is_neg,
            len,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wire {
    pub flag: Flag,
    pub is_neg: bool,
    pub index: usize,
    pub len: usize,
}

impl Wire {
    fn is_input(&self) -> bool {
        self.flag == Flag::A || self.flag == Flag::B
    }

    fn is_output(&self) -> bool {
        self.flag == Flag::S
    }

    fn is_input_or_output(&self) -> bool {
        self.is_input() || self.is_output()
    }

    fn is_not_input_or_output(&self) -> bool {
        !self.is_input_or_output()
    }

    pub fn index_end(&self) -> usize {
        self.index + 1 - self.len
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        if self.is_not_input_or_output() {
            if self.is_neg {
                ret.push('n');
            }
        }
        ret.push_str(self.flag.to_str());
        if self.len > 1 {
            if self.index + 1 < self.len {
                panic!("bad index and len combination for wire : {:?}", self);
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

    pub fn from_logic_extend(logic_extend : FlagExtend, index : usize, len : usize) -> Self {
        Wire {
            flag: logic_extend.flag,
            is_neg: logic_extend.is_neg,
            index,
            len,
        }
    }

    pub fn is_logic_equil(&self, other: &Wire) -> bool {
        fn equil_map(input: &Wire) -> Wire {
            input.clone()
        }

        equil_map(self) == equil_map(other)
    }

    pub fn to_rev(&self) -> Wire {
        Wire {
            flag: self.flag.clone(),
            is_neg: !self.is_neg,
            index: self.index,
            len: self.len,
        }
    }

    pub fn to_flag_extend(&self) -> FlagExtend {
        FlagExtend {
            flag: self.flag.clone(),
            is_neg: self.is_neg,
        }
    }

    pub fn to_wire_float(&self) -> WireFloat {
        WireFloat {
            flag: self.flag.clone(),
            is_neg: self.is_neg,
            len: self.len,
        }
    }
}

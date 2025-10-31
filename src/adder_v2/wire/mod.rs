pub mod wire_list;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

// Wire但是缺少index和len
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlagP {
    pub flag: Flag,
    pub is_neg: bool,
}

impl FlagP {
    pub fn new(flag: Flag, is_neg: bool) -> Self {
        Self {
            flag,
            is_neg,
        }
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        if self.is_neg {
            ret.push('n');
        }
        ret.push_str(self.flag.to_str());
        ret
    }
    
    pub fn to_rev(&self) -> Self {
        Self {
            flag: self.flag.clone(),
            is_neg: !self.is_neg,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FlagPM {
    pub flag: Flag,
    pub is_neg: bool,
    pub is_mirror: bool,
}

impl FlagPM {
    pub fn from_flag_p(flag_p: &FlagP, is_mirror: bool) -> Self {
        Self {
            flag: flag_p.flag.clone(),
            is_neg: flag_p.is_neg,
            is_mirror,
        }
    }
}

/// Wire但是缺少index和len
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireFloat {
    pub flag: Flag,
    pub is_neg: bool,
    pub is_mirror: bool,
    pub len: usize,
}

impl WireFloat {
    pub fn from_str(s : &str) -> Self {
        let (is_neg, mut remained) = if s.starts_with("n") {
            (true, s[1..].to_string())
        } else {
            (false, s.to_string())
        };

        let (is_mirror, mut remained) = if remained.starts_with("m") {
            (true, remained[1..].to_string())
        } else {
            (false, remained.to_string())
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
            is_mirror,
            len,
        }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Wire {
    pub flag: Flag,
    pub is_neg: bool,
    pub is_mirror: bool,
    pub index: usize,
    pub len: usize,
}

impl Wire {
    pub fn new(flag: Flag, is_neg: bool, index: usize, len: usize) -> Self {
        Self {
            flag,
            is_neg,
            is_mirror: false,
            index,
            len,
        }
    }
    pub fn is_input(&self) -> bool {
        self.flag == Flag::A || self.flag == Flag::B
    }

    fn is_output(&self) -> bool {
        self.flag == Flag::S
    }

    pub fn is_c(&self) -> bool {
        self.flag == Flag::G || self.index +1 == self.len
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
            if self.is_mirror {
                ret.push('m');
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

        let (is_mirror, mut remained) = if remained.starts_with("m") {
            (true, remained[1..].to_string())
        } else {
            (false, remained.to_string())
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
            is_mirror,
            index,
            len,
        }
    }

    pub fn from_wire_float(wire_float : WireFloat, index : usize) -> Self {
        Wire {
            flag: wire_float.flag,
            is_neg: wire_float.is_neg,
            is_mirror: wire_float.is_mirror,
            index,
            len: wire_float.len,
        }
    }

    pub fn from_flag_p(logic_extend : FlagP, index : usize, len : usize) -> Self {
        Wire {
            flag: logic_extend.flag,
            is_neg: logic_extend.is_neg,
            is_mirror: false,
            index,
            len,
        }
    }

    pub fn from_flag_pm(logic_extend : FlagPM, index : usize, len : usize) -> Self {
        Wire {
            flag: logic_extend.flag,
            is_neg: logic_extend.is_neg,
            is_mirror: logic_extend.is_mirror,
            index,
            len,
        }
    }

    pub fn is_logic_equil(&self, other: &Wire) -> bool {
        fn equil_map(input: &Wire) -> Wire {
            // 镜像等价，还原到非镜像的情况
            if input.is_mirror {
                let mut ret = input.to_mirror();
                if input.flag == Flag::Q && input.len == 1 {
                    return ret.to_rev();
                }
                if input.flag == Flag::G && input.len == 1 {
                    ret.flag = Flag::P;
                    return ret;
                }
            }

            input.clone()
        }

        equil_map(self) == equil_map(other)
    }

    pub fn to_rev(&self) -> Wire {
        Wire {
            flag: self.flag.clone(),
            is_neg: !self.is_neg,
            is_mirror: self.is_mirror,
            index: self.index,
            len: self.len,
        }
    }

    pub fn to_mirror(&self) -> Wire {
        Wire {
            flag: self.flag.clone(),
            is_neg: self.is_neg,
            is_mirror: !self.is_mirror,
            index: self.index,
            len: self.len,
        }
    }

    pub fn to_mirror_if(&self, is : bool) -> Wire {
        if is {
            self.to_mirror()
        } else {
            self.clone()
        }
    }

    pub fn to_flag_p(&self) -> FlagP {
        FlagP {
            flag: self.flag.clone(),
            is_neg: self.is_neg,
        }
    }

    pub fn to_flag_pm(&self) -> FlagPM {
        FlagPM {
            flag: self.flag.clone(),
            is_neg: self.is_neg,
            is_mirror: self.is_mirror,
        }
    }

    pub fn to_wire_float(&self) -> WireFloat {
        WireFloat {
            flag: self.flag.clone(),
            is_neg: self.is_neg,
            is_mirror: self.is_mirror,
            len: self.len,
        }
    }
}


#[cfg(test)]
mod test_wire {
    use crate::adder_v2::wire::Wire;

    #[test]
    fn test_wire_s() {
        let wire = Wire::from_str("s0");
        dbg!(&wire);
        println!("{}", wire.to_string());
    }
}
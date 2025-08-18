use std::fmt::Debug;

use crate::std::wire::{Flag, Wire};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AmbiguousWire {
    Precise(Wire),
    MayLenNotGivenOrSearchMax { flag : Flag, is_neg : bool, index : usize, may_len : Option<usize> },
    LenGivenByOrder { flag : Flag, is_neg : bool, index : usize, len_choice : Vec<usize> },
}

impl Wire {
    pub fn to_ambiguous(self) -> AmbiguousWire {
        AmbiguousWire::Precise(self)
    }
}

impl AmbiguousWire {
    pub fn to_string(&self) -> String {
        match self {
            AmbiguousWire::Precise(w) => w.to_string(),
            AmbiguousWire::MayLenNotGivenOrSearchMax { flag, is_neg, index, may_len } => {
                if let Some(len) = may_len {
                    Wire {
                        flag: *flag,
                        is_neg: *is_neg,
                        index: *index,
                        len: *len,
                    }.to_string()
                } else {
                    let mut ret = String::new();
                    if *is_neg {
                        ret.push('n');
                    }
                    ret.push_str(flag.to_str());
                    ret.push_str(&format!("{}", index));
                    ret.push_str("..?");

                    ret
                }
            },
            AmbiguousWire::LenGivenByOrder { flag, is_neg, index, len_choice } => {
                let mut ret = String::new();
                for len in len_choice {
                    ret.push_str(&Wire {
                        flag: *flag,
                        is_neg: *is_neg,
                        index: *index,
                        len: *len,
                    }.to_string());
                    ret += " | ";
                }
                ret
            }
        }
    }
}

impl Debug for AmbiguousWire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
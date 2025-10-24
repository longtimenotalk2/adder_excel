use std::{collections::BTreeMap, ops::RangeInclusive};

use crate::adder_v2::{node::{FlagPChain, Node}, wire::{wire_list::WireList, Flag, FlagP, Wire}};

/*
f : flag
p : polar
m : mirror
i : index
l : len
问题分解
纯逻辑层：不考虑极性和Mirror，只考虑GHPQ
镜像层：考虑Mirror
极性层：考虑极性

匹配规则：
每个FlagP是确定的

范围分大尺度和小尺度（小尺度浮动1，由Flag决定）

大尺度下有两个index，称为index1和index2
index1为白色棋子的帽子
index2为内层灰色棋子

大模式有生成三角（G、H）还是生成纯灰（P、Q）

纯灰（P、Q）P模式下，
不考虑index1
index2依次后推

三角（G、H）模式下
G和h的len推index1
P和Q的len推index2

能不能通过球的逻辑直接强解？

本文件只返回：默认输入都是正，输出都是反的逻辑块。当然寻找wire时是看input的
输入：各个FlagP（里面包含is_neg）
输出：使用的基本逻辑以及每个位置装什么，或者无法匹配信息
*/

// Wire 但是起始和终点是一个范围
pub struct WireRange {
    pub flag : Flag,
    pub is_neg : bool,
    pub index_range : RangeInclusive<usize>,
    pub end_index_range : RangeInclusive<usize>,
}

impl WireRange {
    pub fn to_flag_p(&self) -> FlagP {
        FlagP {
            flag : self.flag.clone(),
            is_neg : self.is_neg,
        }
    }
    pub fn from_flag_extand(flag_extand : &FlagP, index_range : RangeInclusive<usize>, end_index_range : RangeInclusive<usize>) -> Self {
        Self {
            flag : flag_extand.flag.clone(),
            is_neg : flag_extand.is_neg,
            index_range,
            end_index_range,
        }
    }
}

#[derive(Debug, Clone)]
enum ExtendGrey {
    Extend(usize),
    ShrinkOne,
}

/*
w
gw
ggw
gggw
ggggw

   index
extend grey = 3
gggw
ggggw
gggggw

w
 w
 gw
 ggw

extend grey = Shrink1

*/

#[derive(Debug, Clone)]
struct BallonG {
    index : usize,
    index_end : usize,
    extend_grey : ExtendGrey
}

#[derive(Debug, Clone)]
struct BallenP {
    index : usize,
    index_end : usize,
    is_end_q : bool,
}

#[derive(Debug, Clone)]
enum Ballen {
    G(BallonG),
    P(BallenP),
}

#[derive(Debug, Clone)]
pub struct FlagIndexLen {
    pub flag : Flag,
    pub index : usize,
    pub len : usize,
}

impl FlagIndexLen {
    pub fn index_end(&self) -> usize {
        self.index + 1 - self.len
    }
}

impl Ballen {
    fn index(&self) -> usize {
        match self {
            Self::G(ballon_g) => ballon_g.index,
            Self::P(ballon_p) => ballon_p.index,
        }
    }

    fn index_end(&self) -> usize {
        match self {
            Self::G(ballon_g) => ballon_g.index_end,
            Self::P(ballon_p) => ballon_p.index_end,
        }
    }

    fn from_flag_index_len(fil : &FlagIndexLen) -> Self {
        let index = fil.index;
        let index_end = fil.index_end();
        match &fil.flag {
            Flag::G => Self::G(BallonG {
                index,
                index_end,
                extend_grey : ExtendGrey::Extend(0),
            }), 
            Flag::P => Self::P(BallenP {
                index,
                index_end,
                is_end_q : false,
            }),
            Flag::Q => Self::P(BallenP {
                index,
                index_end,
                is_end_q : true,
            }),
            Flag::H => Self::G(BallonG {
                index,
                index_end,
                extend_grey : ExtendGrey::ShrinkOne,
            }),
            _ => panic!("can not ballen {:?}", fil),
        }
    }

    fn check_if_valid_now(&self, flag_now : &Flag) -> bool {
        match flag_now {
            Flag::G => {
                match self {
                    Self::G(bollen_g) => match bollen_g.extend_grey {
                        ExtendGrey::Extend(i) => i == 0,
                        ExtendGrey::ShrinkOne => false,
                    },
                    Self::P(_) => false,
                }
            }, 
            Flag::H => {
                match self {
                    Self::G(bollen_g) => match bollen_g.extend_grey {
                        ExtendGrey::Extend(_) => false,
                        ExtendGrey::ShrinkOne => true,
                    },
                    Self::P(_) => false,
                }
            },
            Flag::P => {
                match self {
                    Self::G(bollen_g) => match bollen_g.extend_grey {
                        ExtendGrey::Extend(_) => true,
                        ExtendGrey::ShrinkOne => false,
                    },
                    Self::P(_) => true,
                }
            },
            Flag::Q => {
                match self {
                    Self::G(bollen_g) => match bollen_g.extend_grey {
                        ExtendGrey::Extend(i) => i > 0,
                        ExtendGrey::ShrinkOne => false,
                    },
                    Self::P(_) => true,
                }
            },
            _ => unimplemented!()
        }
    }

    fn give_next_range(&self, flag_now : &Flag, flag_next : Option<&Flag>) -> (RangeInclusive<usize>, RangeInclusive<usize>, Option<AOLogic>) {
        if !self.check_if_valid_now(flag_now) {
            panic!("for bollen now {self:?}, can not parse flag {flag_now:?}")
        }
        if let Some(flag_next) = flag_next {
            match flag_now {
                Flag::G => {
                    match flag_next {
                        Flag::P | Flag::Q => {
                            (self.index()..=self.index(), (self.index_end()+1)..=(self.index_end()+1), Some(AOLogic::Or))
                        },
                        _ => unimplemented!()
                    }
                }
                Flag::H => {
                    match flag_next {
                        Flag::P | Flag::Q => {
                            (self.index()..=self.index(), (self.index_end()+1)..=(self.index_end()+1), Some(AOLogic::Or))
                        },
                        _ => unimplemented!()
                    }
                }
                Flag::P | Flag::Q => {
                    match self {
                        Self::G(bollen_g) => {
                            if let ExtendGrey::Extend(extend_grey) = bollen_g.extend_grey {
                                match (flag_now, flag_next) {
                                    (Flag::P, Flag::G) => {
                                        (
                                            (self.index()+extend_grey)..=(self.index()+extend_grey), 
                                            (self.index())..=(self.index()+1),
                                           Some( AOLogic::And), 
                                        )
                                    },
                                    (Flag::P, Flag::H) => {
                                        (
                                            (self.index()+extend_grey)..=(self.index()+extend_grey), 
                                            (self.index())..=(self.index()),
                                            Some(AOLogic::And), 
                                        )
                                    },
                                    (Flag::Q, Flag::G) => {
                                        (
                                            (self.index()+extend_grey)..=(self.index()+extend_grey), 
                                            (self.index()+1)..=(self.index()+1),
                                            Some(AOLogic::And), 
                                        )
                                    },
                                    _ => unimplemented!(),
                                }
                            } else {
                                unimplemented!()
                            }
                        },
                        Self::P(_) => {
                            match flag_next {
                                Flag::P | Flag::Q => {
                                    (self.index()..=self.index(), (self.index_end()+1)..=(self.index_end()+1), Some(AOLogic::And))
                                },
                                _ => unimplemented!()
                            }
                        },
                    }
                }
                _ => unimplemented!()
            }
        } else {
            // 没有下一个时，必须相等
            (self.index()..=self.index(), self.index_end()..=self.index_end(), None)
        }
    }

    fn consume(&mut self, flag : &Flag, len : usize) {
        match flag {
            Flag::G => {
                if let Ballen::G(bollen_g) = self {
                    match &bollen_g.extend_grey {
                        ExtendGrey::Extend(i) => {
                            assert!(*i == 0);
                            bollen_g.index -= len;
                            bollen_g.extend_grey = ExtendGrey::Extend(len);
                        }
                        ExtendGrey::ShrinkOne => unimplemented!()
                    }
                } else {unimplemented!()}
            }
            Flag::H => {
                if let Ballen::G(bollen_g) = self {
                    match &bollen_g.extend_grey {
                        ExtendGrey::Extend(_) => { unimplemented!() }
                        ExtendGrey::ShrinkOne => {
                            bollen_g.index -= len;
                            bollen_g.extend_grey = ExtendGrey::Extend(len-1);
                        }
                    }
                } else {unimplemented!()}
            }
            Flag::P => {
                match self {
                    Ballen::G(bollen_g) => {
                        match &bollen_g.extend_grey {
                            ExtendGrey::Extend(i) => { 
                                if len == *i + 1 {
                                    bollen_g.extend_grey = ExtendGrey::ShrinkOne;
                                } else if len < *i + 1 {
                                    bollen_g.index -= len;
                                } else {
                                    unimplemented!()
                                }
                            }
                            ExtendGrey::ShrinkOne => {
                                unimplemented!()
                            }
                        }
                    },
                    Ballen::P(bollen_p) => {
                        bollen_p.index -= len;
                    },
                }
            },
            Flag::Q => {
                match self {
                    Ballen::G(bollen_g) => {
                        match &bollen_g.extend_grey {
                            ExtendGrey::Extend(i) => { 
                                if len < *i + 1 {
                                    bollen_g.index -= len;
                                } else {
                                    unimplemented!()
                                }
                            }
                            ExtendGrey::ShrinkOne => {
                                unimplemented!()
                            }
                        }
                    },
                    Ballen::P(bollen_p) => {
                        bollen_p.index -= len;
                    },
                }
            },
            _ => unimplemented!()
        }
    }
}

#[derive(Debug, Clone)]
enum AOLogic {
    And,
    Or,
}

struct CalcGHPQ {

}

impl CalcGHPQ {
    pub fn solve(
        fil : &FlagIndexLen,
        fp_chain : &FlagPChain,
        history_wires: &WireList
    ) {
        
    }
}

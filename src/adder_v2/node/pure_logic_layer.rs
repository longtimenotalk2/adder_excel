use std::{collections::BTreeMap, ops::RangeInclusive};

use colorful::{Color, Colorful};

use crate::adder_v2::{logic::Logic, node::{FlagPChain, Node}, wire::{wire_list::WireList, Flag, FlagP, FlagPM, Wire}, Id};

/*
f : flag
p : polar
m : mirror
i : index
l : len
问题分解
纯逻辑层：不考虑极性和Mirror，只考虑GHPQ  pure_logic_layer
镜像层：考虑Mirror  mirror_layer
极性层：考虑极性  polar_layer

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
#[derive(Debug, Clone)]
pub struct WireRange {
    pub flag : Flag,
    pub is_neg : bool,
    pub is_mirror : bool,
    pub index_range : RangeInclusive<usize>,
    pub end_index_range : RangeInclusive<usize>,
}

impl WireRange {
    pub fn new(flag : &Flag, is_neg : bool, is_mirror : bool, index_range : RangeInclusive<usize>, end_index_range : RangeInclusive<usize>) -> Self {
        Self {
            flag : flag.clone(),
            is_neg,
            is_mirror,
            index_range,
            end_index_range,
        }
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        if self.is_neg {
            ret.push('n');
        }
        ret.push_str(self.flag.to_str());
        ret.push_str(&format!("({:?})", self.index_range));
        ret.push_str("_");
        ret.push_str(&format!("({:?})", self.end_index_range));

        ret
    }

    pub fn to_flag_p(&self) -> FlagP {
        FlagP {
            flag : self.flag.clone(),
            is_neg : self.is_neg,
        }
    }
    pub fn from_flag_pm(flag_extand : &FlagPM, index_range : RangeInclusive<usize>, end_index_range : RangeInclusive<usize>) -> Self {
        Self {
            flag : flag_extand.flag.clone(),
            is_neg : flag_extand.is_neg,
            is_mirror : flag_extand.is_mirror,
            index_range,
            end_index_range,
        }
    }

    // 在输出的index和len符合自身要求时返回
    pub fn try_to_wire(&self, index : usize, len : usize) -> Option<Wire> {
        let index_end = index + 1 - len;
        if self.index_range.contains(&index) && self.end_index_range.contains(&index_end) {
            Some(Wire {
                flag : self.flag.clone(),
                is_neg : self.is_neg,
                is_mirror : self.is_mirror,
                index,
                len,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
enum ExtendGrey {
    Extend(usize),
    ShrinkOne,
    MayShrinkOne,
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

    pub fn from_wire(wire : &Wire) -> Self {
        Self {
            flag : wire.flag.clone(),
            index : wire.index,
            len : wire.len,
        }
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
                        ExtendGrey::MayShrinkOne => true, 
                    },
                    Self::P(_) => false,
                }
            }, 
            Flag::H => {
                match self {
                    Self::G(bollen_g) => match bollen_g.extend_grey {
                        ExtendGrey::Extend(_) => false,
                        ExtendGrey::ShrinkOne | ExtendGrey::MayShrinkOne => true,
                    },
                    Self::P(_) => false,
                }
            },
            Flag::P => {
                match self {
                    Self::G(bollen_g) => match bollen_g.extend_grey {
                        ExtendGrey::Extend(_) => true,
                        ExtendGrey::ShrinkOne | ExtendGrey::MayShrinkOne => false,
                    },
                    Self::P(_) => true,
                }
            },
            Flag::Q => {
                match self {
                    Self::G(bollen_g) => match bollen_g.extend_grey {
                        ExtendGrey::Extend(i) => i > 0,
                        ExtendGrey::ShrinkOne | ExtendGrey::MayShrinkOne => false,
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
                            (self.index()..=self.index(), (self.index_end()+1)..=(self.index()), Some(AOLogic::Or))
                        },
                        _ => unimplemented!()
                    }
                }
                Flag::H => {
                    match flag_next {
                        Flag::P | Flag::Q => {
                            (self.index()..=self.index(), (self.index_end()+1)..=(self.index()), Some(AOLogic::Or))
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
                                    (self.index()..=self.index(), (self.index_end()+1)..=(self.index()), Some(AOLogic::And))
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
                        ExtendGrey::MayShrinkOne => {
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
                        ExtendGrey::ShrinkOne | ExtendGrey::MayShrinkOne => {
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
                                    bollen_g.extend_grey = ExtendGrey::MayShrinkOne;
                                } else if len == *i  {
                                    bollen_g.extend_grey = ExtendGrey::Extend(0);
                                } else {
                                    unimplemented!()
                                }
                            }
                            ExtendGrey::ShrinkOne | ExtendGrey::MayShrinkOne => {
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
                                if len == *i  {
                                    bollen_g.extend_grey = ExtendGrey::Extend(0);
                                } else {
                                    unimplemented!()
                                }
                            }
                            ExtendGrey::ShrinkOne | ExtendGrey::MayShrinkOne=> {
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


#[derive(Debug, Clone)]
pub struct FailParse {
    founded : Vec<(Id, Wire)>,
    not_founded : WireRange,
}

impl FailParse {
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s += "  found : ";
        for found in &self.founded {
            s += &format!("{}, ", found.1.to_string().color(Color::Green));
        }
        s += &format!("; but not found : {}", self.not_founded.to_string().color(Color::Red));
        s
    }
}


impl WireList {
    pub fn solve_pure_logic_layer(
        &self,
        fp_chain : &FlagPChain,
        target_wire : &Wire,
        use_mirror_cell : bool,
    ) -> Result<Node, Vec<FailParse>>{
        let fil = &FlagIndexLen::from_wire(target_wire);
        let ballen = Ballen::from_flag_index_len(fil);
        let mut fail_parse_list: Vec<FailParse> = vec![];

        // let logic_chain = vec![];
        // let founded_wires = vec![];

        fn solve_flags(
            ballen : &Ballen,
            fpm_chain : &[FlagPM],
            wire_list : &WireList,
            history_find_wire : &[(Id, Wire)],
            history_aologic : &[AOLogic],
            fail_parse_list : &mut Vec<FailParse>,
            iter : &mut usize,
        ) -> Result<(Vec<(Id, Wire)>, Vec<AOLogic>), ()> {
            *iter += 1;
            if *iter > 100 {
                panic!("loop over 100 !")
            }

            let flag_now = &fpm_chain[0].flag;
            let is_neg = fpm_chain[0].is_neg;
            let is_mirror = fpm_chain[0].is_mirror;
            let flag_next = fpm_chain.get(1).map(|fp| &fp.flag);

            let (range_start, range_end, next_logic) = ballen.give_next_range(flag_now, flag_next);

            let wire_range_to_find = WireRange::new(flag_now, is_neg, is_mirror, range_start, range_end);
            let all_find = wire_list.find_wire_range(&wire_range_to_find);
            if all_find.is_empty() {
                fail_parse_list.push(FailParse { founded: history_find_wire.to_vec(), not_founded: wire_range_to_find });
                return Err(());
            } else {
                for (id, new_wire) in &all_find {
                    let mut history_find_wire = history_find_wire.clone().to_vec();
                    history_find_wire.push((*id, new_wire.clone()));
                    if flag_next.is_none() {
                        return Ok((history_find_wire, history_aologic.to_vec()));
                    } else {
                        let mut history_aologic = history_aologic.to_vec();
                        history_aologic.push(next_logic.clone().unwrap());
                        let mut ballen = ballen.clone();
                        // dbg!(&history_find_wire);
                        ballen.consume(&flag_now, new_wire.len);
                        if let Ok(ret) = solve_flags(&ballen, &fpm_chain[1..], wire_list, &history_find_wire, &history_aologic, fail_parse_list, iter) {
                            return Ok(ret);
                        }
                    }
                    
                }
                return Err(());
            }
        }

        let mut fpm_chain = vec![];
        for (i, fp) in fp_chain.0.iter().enumerate() {
            if i != fp_chain.0.len() - 1 {
                fpm_chain.push(FlagPM::from_flag_p(fp, use_mirror_cell));
            } else {
                match target_wire.flag {
                    Flag::P | Flag::Q => {
                        if target_wire.is_mirror != use_mirror_cell {
                            panic!("{} : for create P or Q, cell mirror must equal to out wire mirror, cell = {}, wire = {}", target_wire.to_string(), use_mirror_cell, target_wire.is_mirror);
                        }
                        fpm_chain.push(FlagPM::from_flag_p(fp, use_mirror_cell));
                    }
                    Flag::G | Flag::H => {
                        fpm_chain.push(FlagPM::from_flag_p(fp, target_wire.is_mirror));
                    }
                    _ => unimplemented!()
                }
            }
        }

        let result = solve_flags(&ballen, &fpm_chain, self, &[], &[], &mut fail_parse_list, &mut 0);
        if let Ok((mut wires, aologics)) = result {
            let mut logic = Logic::parse_from_aologic(&aologics);
            if use_mirror_cell {
                logic = logic.mirror();
            }
            wires.push((self.0.len() as Id, target_wire.clone()));
            Ok(Node::create_by_ordered_wires(logic, wires))
        } else {
           Err(fail_parse_list)
        }
    }
}

impl Logic {
    fn parse_from_aologic(aologic : &[AOLogic]) -> Self {
        use AOLogic::And as A;
        use AOLogic::Or as O;
        match aologic {
            &[A] => Logic::ND2,
            &[O] => Logic::NR2,
            &[A, O] => Logic::OAI21,
            &[O, A] => Logic::AOI21,
            &[A, O, A] => Logic::AOAI211,
            &[O, A, O] => Logic::OAOI211,
            _ => panic!("can not parse logic {aologic:?}")
        }
    }
}

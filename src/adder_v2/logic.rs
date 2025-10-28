use std::collections::{BTreeMap, BTreeSet};

use crate::adder_v2::{wire::Wire, Id, Port};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Logic {
    INV,
    ND2,
    NR2,
    AN2,
    OR2,
    IND2,
    INR2,
    XOR2,
    XNR2,
    XOR2DOUT,
    XNR2DOUT,
    AOI21,
    OAI21,
    AO21,
    OA21,
    IAOI21,
    IOAI21,
    AOI22,
    OAI22,
    AOAI211,
    OAOI211,
    AOA211,
    OAO211,
}



impl Logic {
    pub fn to_string(&self) -> &'static str {
        match self {
            Logic::INV => "INV",
            Logic::ND2 => "ND2",
            Logic::NR2 => "NR2",
            Logic::AN2 => "AN2",
            Logic::OR2 => "OR2",
            Logic::IND2 => "IND2",
            Logic::INR2 => "INR2",
            Logic::XOR2 => "XOR2",
            Logic::XNR2 => "XNR2",
            Logic::XOR2DOUT => "XOR2DOUT",
            Logic::XNR2DOUT => "XNR2DOUT",
            Logic::AOI21 => "AOI21",
            Logic::OAI21 => "OAI21",
            Logic::AO21 => "AO21",
            Logic::OA21 => "OA21",
            Logic::IAOI21 => "IAOI21",
            Logic::IOAI21 => "IOAI21",
            Logic::AOI22 => "AOI22",
            Logic::OAI22 => "OAI22",
            Logic::AOAI211 => "AOAI211",
            Logic::OAOI211 => "OAOI211",
            Logic::AOA211 => "AOA211",
            Logic::OAO211 => "OAO211",
        }
    }

    pub fn z_port(&self) -> Port {
        match self {
            Logic::AN2 | Logic::OR2 | Logic::XOR2 | Logic::XOR2DOUT | Logic::AO21 | Logic::OA21 | Logic::AOA211 | Logic::OAO211 => Port::new("Z"),
            _ => Port::new("ZN"),
        }
    }

    pub fn mirror(&self) -> Self {
        match self {
            Logic::INV => Logic::INV,
            Logic::ND2 => Logic::NR2,
            Logic::NR2 => Logic::ND2,
            Logic::AN2 => Logic::OR2,
            Logic::OR2 => Logic::AN2,
            Logic::IND2 => Logic::INR2,
            Logic::INR2 => Logic::IND2,
            Logic::XOR2 => Logic::XNR2,
            Logic::XNR2 => Logic::XOR2,
            Logic::XOR2DOUT => Logic::XNR2DOUT,
            Logic::XNR2DOUT => Logic::XOR2DOUT,
            Logic::AOI21 => Logic::OAI21,
            Logic::OAI21 => Logic::AOI21,
            Logic::AO21 => Logic::OA21,
            Logic::OA21 => Logic::AO21,
            Logic::IAOI21 => Logic::IOAI21,
            Logic::IOAI21 => Logic::IAOI21,
            Logic::AOI22 => Logic::OAI22,
            Logic::OAI22 => Logic::AOI22,
            Logic::AOAI211 => Logic::OAOI211,
            Logic::OAOI211 => Logic::AOAI211,
            Logic::AOA211 => Logic::OAO211,
            Logic::OAO211 => Logic::AOA211,
        }
    }

    pub fn input_port_ordered(&self) -> Vec<Port> {
        let i = Port::new("I");
        let a1 = Port::new("A1");
        let a2 = Port::new("A2");
        let b = Port::new("B");
        let b1 = Port::new("B1");
        let b2 = Port::new("B2");
        let c = Port::new("C");
        match self {
            Logic::INV => vec![i],
            Logic::ND2 | Logic::NR2 | Logic::AN2 | Logic::OR2 | Logic::XNR2 | Logic::XNR2DOUT | Logic::XOR2 | Logic::XOR2DOUT => vec![a1, a2],
            Logic::AOI21 | Logic::OAI21 | Logic::IAOI21 | Logic::IOAI21 | Logic::AO21 | Logic::OA21 => vec![b, a1, a2],
            Logic::AOAI211 | Logic::OAOI211 | Logic::AOA211 | Logic::OAO211 => vec![c, b, a1, a2],
            Logic::IND2 | Logic::INR2 => vec![a1, b1],
            Logic::AOI22 | Logic::OAI22 => vec![a1, a2, b1, b2],
        }
    }

    // return z* and o1?
    pub fn calc(&self, inputs : &BTreeMap<Port, bool>) -> (bool, Option<bool>) {
        match self {
            Logic::XOR2DOUT => {
                let a1 = *inputs.get(&Port::new("A1")).unwrap();
                let a2 = *inputs.get(&Port::new("A2")).unwrap();
                let o1 = !(a1 || a2);
                let z = a1 ^ a2;
                (z, Some(o1))
            }
            Logic::XNR2DOUT => {
                let a1 = *inputs.get(&Port::new("A1")).unwrap();
                let a2 = *inputs.get(&Port::new("A2")).unwrap();
                let o1 = !(a1 && a2);
                let zn = !a1 ^ a2;
                (zn, Some(o1))
            }
            _ => {
                let out = match self {
                    Logic::INV => {
                        let i = *inputs.get(&Port::new("I")).unwrap();
                        !i
                    },
                    Logic::ND2 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        !(a1 && a2)
                    },
                    Logic::NR2 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        !(a1 || a2)
                    },
                    Logic::AN2 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        a1 && a2
                    },
                    Logic::OR2 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        a1 || a2
                    },
                    Logic::IND2 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let b1 = *inputs.get(&Port::new("B1")).unwrap();
                        !(!a1 && b1)
                    },
                    Logic::INR2 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let b1 = *inputs.get(&Port::new("B1")).unwrap();
                        !(!a1 || b1)
                    },
                    Logic::XOR2 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        a1 ^ a2
                    },
                    Logic::XNR2 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        !a1 ^ a2
                    },
                    Logic::XOR2DOUT => unimplemented!(),
                    Logic::XNR2DOUT => unimplemented!(),
                    Logic::AOI21 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        !((a1 && a2) || b)
                    },
                    Logic::OAI21 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        !((a1 || a2) && b)
                    },
                    Logic::AO21 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        (a1 && a2) || b
                    },
                    Logic::OA21 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        (a1 || a2) && b
                    },
                    Logic::IAOI21 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        !((a1 && !a2) || b)
                    },
                    Logic::IOAI21 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        !((a1 || !a2) && b)
                    },
                    Logic::AOI22 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b1 = *inputs.get(&Port::new("B1")).unwrap();
                        let b2 = *inputs.get(&Port::new("B2")).unwrap();
                        !((a1 && a2) || (b1 && b2))
                    },
                    Logic::OAI22 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b1 = *inputs.get(&Port::new("B1")).unwrap();
                        let b2 = *inputs.get(&Port::new("B2")).unwrap();
                        !((a1 || a2) && (b1 || b2))
                    },
                    Logic::AOAI211 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        let c = *inputs.get(&Port::new("C")).unwrap();
                        !(((a1 && a2) || b) && c)
                    },
                    Logic::OAOI211 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        let c = *inputs.get(&Port::new("C")).unwrap();
                        !(((a1 || a2) && b) || c)
                    },
                    Logic::AOA211 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        let c = *inputs.get(&Port::new("C")).unwrap();
                        ((a1 && a2) || b) && c
                    },
                    Logic::OAO211 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        let c = *inputs.get(&Port::new("C")).unwrap();
                        ((a1 || a2) && b) || c
                    },
                };
                (out, None)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct IO<T> {
    pub input : BTreeMap<Port, T>,
    pub output_z : T,
    pub output_o1 : Option<T>,
}

impl<T> IO<T> {
    pub fn new(input : BTreeMap<Port, T>, output_z : T, output_o1 : Option<T>) -> Self {
        IO {
            input,
            output_z,
            output_o1,
        }
    }
}
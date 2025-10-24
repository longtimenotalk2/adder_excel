use std::collections::BTreeMap;

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
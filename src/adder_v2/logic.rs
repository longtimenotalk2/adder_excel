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

#[derive(Debug, Clone)]
pub struct IO<T> {
    input : BTreeMap<Port, T>,
    output_z : T,
    output_o1 : Option<T>,
}

impl Logic {
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

    /// give logic and if input need swap
    pub fn get_ind_inr_from_and(a1_is_neg : bool, a2_is_neg : bool, out_is_neg : bool) -> (Self, bool) {
        assert_ne!(a1_is_neg, a2_is_neg);
        match (a1_is_neg, a2_is_neg, out_is_neg) {
            (true, false, true) => (Logic::IND2, false),
            (false, true, true) =>(Logic::IND2, true),
            (true, false, false) => (Logic::INR2, true),
            (false, true, false) => (Logic::INR2, false),
            _ => unimplemented!()
        }
    }
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
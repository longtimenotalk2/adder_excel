use crate::std::logic_block::custom::CustomLogicBlock;

pub mod port;
pub mod basic;
pub mod function;
pub mod custom;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LogicBlock {
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
    Custom(CustomLogicBlock)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Port(String);
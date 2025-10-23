pub mod node_create;

use crate::adder_v2::{logic::{Logic, IO}, wire::{Flag, FlagExtend, Wire}, Id};

#[derive(Debug, Clone)]
pub enum Drive {
    D1,
    D2,
}

#[derive(Debug, Clone)]
pub struct Node {
    logic : Logic,
    io : IO<(Id, Wire)>,
    drive : Drive,
}

#[derive(Debug, Clone)]
pub struct FlagExtendChain(pub Vec<FlagExtend>);


#[derive(Debug, Clone)]
pub struct NodeHint {
    is_simple_inv : bool,
    is_start : bool,
    is_start_xnr_dout : bool,
    is_start_xor_dout : bool,
    is_start_xnr : bool,
    is_start_xor : bool,
    drive : Drive,
    given_out_flag_extend : Option<FlagExtend>,
    given_out_index : usize,
    given_out_len : usize,
    given_flag_extend_chain : Option<FlagExtendChain>, 
    is_out_addition_inv : bool,
}


impl Node {
    pub fn new(logic : Logic, io : IO<(Id, Wire)>, drive : Drive) -> Self {
        Self {
            logic,
            io,
            drive,
        }
    }
}
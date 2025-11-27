use std::collections::BTreeSet;

pub mod wire;
pub mod logic;
pub mod node;
pub mod adder;
pub mod excel;
pub mod project;
pub mod cell_parse;
pub mod cdl;
pub mod cell;
pub mod special;
pub mod draw;
pub mod netlist;
pub mod floorplan;

type Id = u32;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Port(pub String);

impl Port {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

pub fn show_string_iter_btreeset(list : BTreeSet<String>) {
    for item in list.iter() {
        println!("{}", item.to_string());
    }
}

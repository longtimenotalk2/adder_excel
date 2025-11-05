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

type Id = u32;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Port(pub String);

impl Port {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}
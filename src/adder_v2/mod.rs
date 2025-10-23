pub mod wire;
pub mod logic;
pub mod node;

type Id = u32;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Port(pub String);

impl Port {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}
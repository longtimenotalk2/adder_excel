use crate::adder_v2::adder::{Cell, CellBody};

pub mod n3e;

pub struct ReadCellName(pub String);

#[derive(Debug, Clone, Copy)]
pub enum ReadCellType {
    Std,
    Custom,
    Lhw,
    LocalHack,
}

#[derive(Debug, Clone, Copy)]
pub enum Process {
    N3E,
}

impl CellBody {
    pub fn parse(&self, process : Process) -> (ReadCellName, ReadCellType) {
        match process {
            Process::N3E => self.parse_n3e(),
        }
    }

    pub fn spf_path(&self, process : Process) -> String {
        match process {
            Process::N3E => self.spf_path_n3e(),
        }
    }
}
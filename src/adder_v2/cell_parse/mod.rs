use crate::adder_v2::{adder, cell::cell_body::CellBody};

pub mod n3e;

pub struct ReadCellName(pub String);

impl ReadCellName {
    pub fn new(name : &str) -> Self {
        Self(name.to_string())
    }
}

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

impl Process {
    pub fn path_base(&self) -> &'static str {
        match self {
            Process::N3E => "/ic/projects/BM1374",
        }
    }

    pub fn spf_suffix(&self) -> &'static str {
        match self {
            Process::N3E => ".Cbest45.spf",
        }
    }

    pub fn lhw_path(&self) -> &'static str {
        match self {
            Process::N3E => "/ic/projects/BM1374/users/haiwei.li/V0/work/spf/out",
        }
    }

    pub fn cds_path(&self) -> &'static str {
        match self {
            Process::N3E => "/ic/projects/BM1374/users/haiwei.li/V0/cds",
        }
    }
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

    pub fn gds_path(&self, process : Process) -> String {
        match process {
            Process::N3E => self.gds_path_n3e(),
        }
    }
}
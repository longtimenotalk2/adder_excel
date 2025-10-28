use std::collections::BTreeSet;

use crate::adder_v2::{adder::Adder, cell::{cell_body::CellBody, cell_info::CellInfo}, cell_parse::Process, Port};

pub mod vddh;

impl CellBody {
    pub fn pg_ports(&self, _process : Process) -> BTreeSet<Port> {
        let mut pg_ports = BTreeSet::new();
        pg_ports.insert(Port::new("VBB"));
        pg_ports.insert(Port::new("VDD"));
        pg_ports.insert(Port::new("VPP"));
        pg_ports.insert(Port::new("VSS"));

        if self.info.is_power_vddh() {
            pg_ports.insert(Port::new("VDDH"));
        }
        pg_ports
    }
}

impl Adder {
    pub fn pg_ports(&self, _process : Process) -> BTreeSet<Port> {
        let mut pg_ports = BTreeSet::new();
        for (_, cell) in self.cells.iter() {
            pg_ports.append(&mut cell.to_cell_body().pg_ports(_process));
        }
        pg_ports
    }
}
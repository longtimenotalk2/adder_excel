use crate::{adder_v2::{adder::Adder, cell_parse::Process}, hspice::{line_cap, line_cell, line_end_subckt, line_inc, line_subckt}};

impl Adder {
    pub fn to_cdl(&self, name : &str, process : Process) -> String {
        let mut txt = String::new();

        // include
        for cell_body in self.cell_body_set() {
            txt += &line_inc(&cell_body.spf_path(process))
        }
        txt += "\n";

        let mut ports = vec![];

        for i in 0..self.bits {
            ports.push(format!("A{i}"));
        }
        for i in 0..self.bits {
            ports.push(format!("B{i}"));
        }
        for i in 0..self.bits {
            ports.push(format!("S{i}"));
        }
        for pg_port in self.pg_ports(process) {
            ports.push(pg_port.0);
        }

        txt += &line_subckt(name, &ports);

        let layer_map = self.scan_layer_absolute();

        for (i, (_, cell)) in self.cells.iter().enumerate() {
            let layer = layer_map[i];

            let mut port_and_pins = cell.node.to_port_vs_wire_name();

            for pg_port in cell.to_cell_body().pg_ports(process) {
                port_and_pins.insert(pg_port.clone(), pg_port.0);
            }

            let pins = port_and_pins.values().collect::<Vec<_>>();

            txt += &line_cell(&cell.node.to_inst_name_with_layer(layer), &pins, &cell.to_cell_body().parse(process).0.0);
        }

        txt += &line_end_subckt();

        txt
    }
}
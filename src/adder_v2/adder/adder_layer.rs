use std::collections::BTreeMap;

use crate::adder_v2::{adder::Adder, Id};

impl Adder {
    pub fn scan_layer_absolute(&self) -> Vec<i32> {
        let mut cell_layer = vec![];
        let mut wire_layer : BTreeMap<Id, i32> = BTreeMap::new();

        for (id, wire) in self.wires.iter() {
            if wire.is_input() {
                wire_layer.insert(*id, 0);
            }
        }

        for (_, cell) in self.cells.iter() {
            let mut layer: i32 = -1;
            for (_, (id, _)) in &cell.node.io.input {
                layer = *wire_layer.get(id).unwrap().max(&layer);
            }
            assert!(layer >= 0);
            let this_layer = layer + 1;
            cell_layer.push(this_layer);
            wire_layer.insert(cell.node.io.output_z.0, this_layer);
            if let Some(output_o1) = &cell.node.io.output_o1 {
                wire_layer.insert(output_o1.0, this_layer);
            }
        }

        cell_layer
    }
}
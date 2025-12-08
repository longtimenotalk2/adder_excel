use std::collections::BTreeMap;

use crate::adder_v2::floorplan_v::{AdderFPMain, SuperParameters};

impl AdderFPMain {
    fn once_force_move_x(&mut self, beta : f64, super_parameters : &SuperParameters) {
        let mut force_x_dict = BTreeMap::new();
        for cell_id in self.all_moveable_cell_ids() {
            let force_x = self.given_cell_x_force(cell_id, super_parameters);
            force_x_dict.insert(cell_id, force_x);
        }
        for cell_id in self.all_moveable_cell_ids() {
            let force_x = force_x_dict.get(&cell_id).unwrap();
            let displacement_x = beta * force_x;
            self.impl_cell_x_movement(cell_id, displacement_x);
        }
    }

    pub fn dynamic_main(&mut self, super_parameters : &SuperParameters) {
        let beta = 0.01;
        let n = 1;
        for _ in 0..n {
            self.once_force_move_x(beta, super_parameters);
        }
    }
}
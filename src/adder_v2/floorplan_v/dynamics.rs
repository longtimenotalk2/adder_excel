use std::collections::BTreeMap;

use crate::adder_v2::floorplan_v::{AdderFPMain, SuperParameters};

impl AdderFPMain {
    fn once_force_move_x(&mut self, beta : f64, super_parameters : &SuperParameters) {
        let mut disp_x_dict = BTreeMap::new();
        let n = 4;
        for cell_id in self.all_moveable_cell_ids() {
            // let force_x = self.given_cell_x_wire_force_atom(cell_id);
            let force_x = self.given_cell_x_force(cell_id, super_parameters);
            let force_x = if force_x == 0. {
                0.
            } else {
                force_x / force_x.abs() * beta
            };
            let disp_x = self.given_cell_x_itered_displacement(cell_id, force_x * beta, n, super_parameters);
            disp_x_dict.insert(cell_id, disp_x);
        }
        for cell_id in self.all_moveable_cell_ids() {
            let disp_x = *disp_x_dict.get(&cell_id).unwrap();
            self.impl_cell_x_movement(cell_id, disp_x);
        }
    }

    pub fn dynamic_main(&mut self, 
        beta : f64,
        super_parameters : &SuperParameters
    ) {
        let n = 1;
        for _ in 0..n {
            self.once_force_move_x(beta, super_parameters);
        }
    }
}
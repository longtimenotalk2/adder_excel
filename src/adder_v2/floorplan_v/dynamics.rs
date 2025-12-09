use std::collections::BTreeMap;

use crate::adder_v2::floorplan_v::{AdderFPMain, CellId, SubAreaId, SuperParameters, YMove};

impl AdderFPMain {
    fn once_force_move_x(&mut self, beta : f64, super_parameters : &SuperParameters) {
        let mut disp_x_dict = BTreeMap::new();
        let n = 4;
        for cell_id in self.all_moveable_cell_ids() {
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

    fn once_force_move_y(&mut self, super_parameters : &SuperParameters) {
        let mut y_move_dict: BTreeMap<CellId, YMove> = BTreeMap::new();
        for cell_id in self.all_moveable_cell_ids() {
            let force_y = self.given_cell_y_force(cell_id, super_parameters);
            for (y_move, force) in force_y {
                if force > 0. {
                    y_move_dict.insert(cell_id, y_move);
                }
            }
        }
        for (cell_id, y_move) in y_move_dict {
            self.impl_cell_y_movement(cell_id, y_move);
        }
    }

    pub fn dynamic_main_x(&mut self, 
        beta : f64,
        super_parameters : &SuperParameters
    ) {
        let n = 1;
        for _ in 0..n {
            self.once_force_move_x(beta, super_parameters);
        }
    }

    pub fn dynamic_main_y(&mut self, 
        super_parameters : &SuperParameters
    ) {
        let n = 1;
        for _ in 0..n {
            self.once_force_move_y(super_parameters);
        }
    }

    pub fn dynamic_combine_5_step(&mut self,
        beta : f64,
        super_parameters : &SuperParameters
    ) {
        self.once_force_move_y(super_parameters);
        for _ in 0..4 {
            self.once_force_move_x(beta, super_parameters);
        }
    }
}
use std::collections::BTreeMap;

use crate::adder_v2::floorplan_v::{AdderFPMain, CellId, SuperParameters, YMove};

const DELTA_X : f64 = 0.1;

impl AdderFPMain {
    pub fn given_cell_x_wire_force_atom(&self, cell_id : CellId) -> f64 {
        let get_energy = |x : f64| {
            let mut new_pos = self.cell_pos_dict.get(&cell_id).unwrap().clone();
            new_pos.impl_x_movement(x);

            self.given_cell_wire_energy_with_new_pos(cell_id, &new_pos)
        };

        let energy_plus = get_energy(DELTA_X);
        let energy_minus = get_energy(-DELTA_X);

        -(energy_plus - energy_minus) / (2. * DELTA_X)
    }

    // down(y--), up(y++) force
    pub fn given_cell_y_wire_force_atom(&self, cell_id : CellId) -> BTreeMap<YMove, f64> {
        let mut ret = BTreeMap::new();
        
        for y_move in YMove::all() {
            if let Some(target_sub_area_id) = self.can_cell_y_move(cell_id, y_move) {
                let energy = self.given_cell_wire_energy(cell_id);
                let mut new_pos = self.cell_pos_dict.get(&cell_id).unwrap().clone();
                new_pos.impl_y_movement(target_sub_area_id);

                let energy_new = self.given_cell_wire_energy_with_new_pos(cell_id, &new_pos);

                let force = -(energy_new - energy);
                ret.insert(y_move, force);
            }
        }

        ret
    }

    // 向一个方向，通过二分的方式，找到使得cell自身能量最优的x位置
    pub fn given_cell_x_itered_displacement(&self, cell_id : CellId, x0 : f64, n_iter : usize, super_parameters : &SuperParameters) -> f64 {
        let mut start_energy = (0., self.given_cell_x_energy(cell_id, super_parameters));
        let mut end_energy = (x0, self.given_cell_x_energy_with_movement(cell_id, x0, super_parameters));
        let mut middle_energy = (x0/2., self.given_cell_x_energy_with_movement(cell_id, x0 / 2., super_parameters));

        for _ in 0..n_iter {
            // if cell_id == CellId(2) {
            //     dbg!(&start_energy);
            //     dbg!(&middle_energy);
            //     dbg!(&end_energy);
            // }
            if start_energy.1 < end_energy.1 {
                end_energy = middle_energy;   
            } else {
                start_energy.1 = middle_energy.1;
            }
            let middle_x = (start_energy.0 + end_energy.0) / 2.;
            middle_energy = (middle_x, self.given_cell_x_energy_with_movement(cell_id, middle_x, super_parameters));
        }

        let mut sorted = vec![start_energy, middle_energy, end_energy];
        sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let ret = sorted.first().unwrap().0;

        // if cell_id == CellId(2) {
        //     dbg!(&sorted);
        //     dbg!(&ret);
        // }

        ret

        
    }

    pub fn given_cell_x_force(&self, cell_id : CellId, super_parameters : &SuperParameters) -> f64 {
        let get_energy = |x : f64| {
            let mut new_pos = self.cell_pos_dict.get(&cell_id).unwrap().clone();
            new_pos.impl_x_movement(x);

            let mut energy = 0.;
            energy += self.given_cell_wire_energy_with_new_pos(cell_id, &new_pos) * super_parameters.alpha_wire_energy;
            energy += self.given_cell_border_energy_with_new_pos(cell_id, &new_pos) * super_parameters.alpha_border_energy;
            energy += self.given_cell_overlap_energy_with_new_pos(cell_id, &new_pos) * super_parameters.alpha_overlap_energy;
            energy
        };

        let energy_plus = get_energy(DELTA_X);
        let energy_minus = get_energy(-DELTA_X);

        -(energy_plus - energy_minus) / (2. * DELTA_X)
    }

    pub fn given_cell_y_force(&self, cell_id : CellId, super_parameters : &SuperParameters) -> BTreeMap<YMove, f64> {
        let energy_base = self.given_cell_y_energy(cell_id, super_parameters);

        let mut ret = BTreeMap::new();

        for y_move in YMove::all() {
            if let Some(target_sub_area_id) = self.can_cell_y_move(cell_id, y_move) {
                let mut new_pos = self.cell_pos_dict.get(&cell_id).unwrap().clone();
                new_pos.impl_y_movement(target_sub_area_id);
                let energy_new = self.given_cell_y_energy_with_new_pos(cell_id, &new_pos, super_parameters);
                let force = -(energy_new - energy_base);
                ret.insert(y_move, force);
            }
        }

        ret

    }
    
}
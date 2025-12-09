use std::collections::BTreeMap;

use crate::adder_v2::floorplan_v::{AdderFPMain, CellId, SuperParameters, YMove};

const DELTA_X : f64 = 0.1;

impl AdderFPMain {
    pub fn given_cell_x_wire_force_atom(&self, cell_id : CellId) -> f64 {
        let get_energy = |x : f64| {
            let mut new_main = self.clone();
            new_main.impl_cell_x_movement(cell_id, x);

            let mut energy = 0.;
            energy += new_main.given_cell_wire_energy(cell_id);
            energy
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
                let mut new_main = self.clone();
                new_main.impl_cell_y_movement(cell_id, y_move);
                let energy_new = new_main.given_cell_wire_energy(cell_id);

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
            let mut new_main = self.clone();
            new_main.impl_cell_x_movement(cell_id, x);

            let mut energy = 0.;
            energy += new_main.given_cell_wire_energy(cell_id) * super_parameters.alpha_wire_energy;
            energy += new_main.given_cell_border_energy(cell_id) * super_parameters.alpha_border_energy;
            energy += new_main.given_cell_overlap_energy(cell_id) * super_parameters.alpha_overlap_energy;
            energy
        };

        let energy_plus = get_energy(DELTA_X);
        let energy_minus = get_energy(-DELTA_X);

        -(energy_plus - energy_minus) / (2. * DELTA_X)
    }

    // pub fn given_cell_y_force(&self, cell_id : CellId, super_parameters : &SuperParameters) -> BTreeMap<YMove, f64> {
    //     let get_energy = |y_move : Option<YMove>| {
    //         let mut new_main = self.clone();
    //         new_main.impl_cell_x_movement(cell_id, x);

    //         let mut energy = 0.;
    //         energy += new_main.given_cell_wire_energy(cell_id) * super_parameters.alpha_wire_energy;
    //         energy += new_main.given_cell_border_energy(cell_id) * super_parameters.alpha_border_energy;
    //         energy += new_main.given_cell_overlap_energy(cell_id) * super_parameters.alpha_overlap_energy;
    //         energy
    //     };

    // }

    // fn given_cell_up_down_force(&self, is_up : bool, cell_id : CellId, super_parameters : &SuperParameters) -> Option<f64> {
    //     let new_sub_area_id = if is_up { self.can_cell_up_move(cell_id) } else { self.can_cell_down_move(cell_id) };
    //     if let Some(sub_area_id_new) = new_sub_area_id {
    //         let sub_area_id_now = self.given_cell_sub_area_id(cell_id);
    //         let mut main_new = self.clone();
    //         if is_up {
    //             main_new.impl_cell_up_move(cell_id);
    //         } else {
    //             main_new.impl_cell_down_move(cell_id);
    //         }
    //         let mut energy_diff = 0.;
    //         // wire energy diff
    //         energy_diff += main_new.given_cell_wire_energy(cell_id) * super_parameters.alpha_wire_energy;
    //         energy_diff -= self.given_cell_wire_energy(cell_id) * super_parameters.alpha_wire_energy;
    //         // density energy diff
    //         energy_diff += main_new.given_sub_area_density_energy(sub_area_id_new) * super_parameters.alpha_density_energy;
    //         energy_diff += main_new.given_sub_area_density_energy(sub_area_id_now) * super_parameters.alpha_density_energy;
    //         energy_diff -= self.given_sub_area_density_energy(sub_area_id_new) * super_parameters.alpha_density_energy;
    //         energy_diff -= self.given_sub_area_density_energy(sub_area_id_now) * super_parameters.alpha_density_energy;
    //         // overlap energy diff
    //         energy_diff += main_new.given_cell_overlap_energy(cell_id) * super_parameters.alpha_overlap_energy;
    //         energy_diff -= self.given_cell_overlap_energy(cell_id) * super_parameters.alpha_overlap_energy;
    //         // border energy diff
    //         energy_diff += main_new.given_cell_border_energy(cell_id) * super_parameters.alpha_border_energy;
    //         energy_diff -= self.given_cell_border_energy(cell_id) * super_parameters.alpha_border_energy;

    //         Some(-energy_diff)

    //     } else {
    //         None
    //     }
    // }

    // pub fn given_cell_up_force(&self, cell_id : CellId, super_parameters : &SuperParameters) -> Option<f64> {
    //     self.given_cell_up_down_force(true, cell_id, super_parameters)
    // }

    // pub fn given_cell_down_force(&self, cell_id : CellId, super_parameters : &SuperParameters) -> Option<f64> {
    //     self.given_cell_up_down_force(false, cell_id, super_parameters)
    // }

    
}
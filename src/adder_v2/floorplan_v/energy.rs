use std::collections::BTreeSet;

use crate::adder_v2::floorplan_v::{AdderFPMain, CellId, CellStaticInfo, ModelParameters, Pos, SubAreaId, SuperParameters, WireId, WireStaticInfo};

fn row_density_function(density : f64) -> f64 {
    let i = (density - 0.8) * 10.;
    f64::exp(i)
}

fn border_energy(border_min : f64, border_max : f64, cell_min : f64, cell_max : f64) -> f64 {
    let mut energy = 0.;
    if cell_min < border_min {
        energy += (border_min - cell_min) * (border_min - cell_min);
    }
    if cell_max > border_max {
        energy += (cell_max - border_max) * (cell_max - border_max);
    }
    energy
}

fn overlap_energy(cell_0_min : f64, cell_0_max : f64, cell_1_min : f64, cell_1_max : f64) -> f64 {
    let mut overlap_len = 0.;
    if cell_0_max > cell_1_min && cell_0_min < cell_1_max {
        overlap_len = (cell_0_max - cell_1_min).min(cell_1_max - cell_0_min);
    }
    overlap_len.powi(2)
}

impl AdderFPMain {

    fn given_wire_energy_inner(&self, wire_id : WireId) -> (i32, i32, f64, f64) {
        let connected_cell_id_set = &self.wire_static_dict.get(&wire_id).unwrap().connected_cells;
        let y_min = connected_cell_id_set.iter().map(|x| self.given_cell_y(*x)).min().unwrap();
        let y_max = connected_cell_id_set.iter().map(|x| self.given_cell_y(*x)).max().unwrap();
        let x_right_min = connected_cell_id_set.iter().map(|x| self.given_cell_x_border(*x).1).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let x_left_max = connected_cell_id_set.iter().map(|x| self.given_cell_x_border(*x).0).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

        (y_min, y_max, x_right_min, x_left_max)
    }

    fn given_wire_energy_inner_with_cell_new_pos(&self, wire_id : WireId, cell_id : CellId, new_pos : &Pos) -> (i32, i32, f64, f64) {
        let connected_cell_id_set = &self.wire_static_dict.get(&wire_id).unwrap().connected_cells;
        let y_min = connected_cell_id_set.iter().map(|x| self.given_cell_y_with_cell_new_pos(*x, cell_id, new_pos)).min().unwrap();
        let y_max = connected_cell_id_set.iter().map(|x| self.given_cell_y_with_cell_new_pos(*x, cell_id, new_pos)).max().unwrap();
        let x_right_min = connected_cell_id_set.iter().map(|x| self.given_cell_x_border_with_cell_new_pos(*x, cell_id, new_pos).1).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let x_left_max = connected_cell_id_set.iter().map(|x| self.given_cell_x_border_with_cell_new_pos(*x, cell_id, new_pos).0).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

        (y_min, y_max, x_right_min, x_left_max)
    }

    pub fn given_wire_energy(&self, wire_id : WireId) -> f64 {
        let (y_min, y_max, x_right_min, x_left_max) = self.given_wire_energy_inner(wire_id);
        let y_diff_abs = (y_max - y_min).abs();
        let x_diff_plus = (x_left_max - x_right_min).max(0.);
        y_diff_abs as f64 * self.model.y_scale + x_diff_plus as f64 * self.model.x_scale
    }

    pub fn given_wire_energy_with_cell_new_pos(&self, wire_id : WireId, cell_id : CellId, new_pos : &Pos) -> f64 {
        let (y_min, y_max, x_right_min, x_left_max) = self.given_wire_energy_inner_with_cell_new_pos(wire_id, cell_id, new_pos);
        let y_diff_abs = (y_max - y_min).abs();
        let x_diff_plus = (x_left_max - x_right_min).max(0.);
        y_diff_abs as f64 * self.model.y_scale + x_diff_plus as f64 * self.model.x_scale
    }

    pub fn given_sub_area_density_energy(&self, sub_area_id : SubAreaId) -> f64 {
        let density = self.given_sub_area_density(sub_area_id);
        let density_energy = row_density_function(density);
        density_energy
    }

    pub fn given_cell_border_energy(&self, cell_id : CellId) -> f64 {
        let (cell_x_min, cell_x_max) = self.given_cell_x_border(cell_id);
        let sub_area_id = self.cell_pos_dict.get(&cell_id).unwrap().sub_area_id;
        let (sub_area_x_min, sub_area_x_max) = self.given_sub_area_x_border(sub_area_id);
        border_energy(sub_area_x_min, sub_area_x_max, cell_x_min, cell_x_max)
    }

    pub fn given_cell_border_energy_with_new_pos(&self, cell_id : CellId, pos: &Pos) -> f64 {
        let (cell_x_min, cell_x_max) = self.given_cell_x_border_with_cell_new_pos(cell_id, cell_id, pos);
        let sub_area_id = pos.sub_area_id;
        let (sub_area_x_min, sub_area_x_max) = self.given_sub_area_x_border(sub_area_id);
        border_energy(sub_area_x_min, sub_area_x_max, cell_x_min, cell_x_max)
    }

    pub fn given_cell_overlap_energy(&self, cell_id : CellId) -> f64 {
        let mut energy = 0.;
        let (cell_x_min, cell_x_max) = self.given_cell_x_border(cell_id);
        let sub_area_id = self.cell_pos_dict.get(&cell_id).unwrap().sub_area_id;
        for other_cell_id in self.filter_cell_id_in_given_sub_area_id(sub_area_id) {
            if other_cell_id != cell_id {
                let (other_cell_x_min, other_cell_x_max) = self.given_cell_x_border(other_cell_id);
                energy += overlap_energy(cell_x_min, cell_x_max, other_cell_x_min, other_cell_x_max);
            }
        }
        energy
    }

    pub fn given_cell_overlap_energy_with_new_pos(&self, cell_id : CellId, pos : &Pos) -> f64 {
        let mut energy = 0.;
        let (cell_x_min, cell_x_max) = self.given_cell_x_border_with_cell_new_pos(cell_id, cell_id, pos);
        let sub_area_id = pos.sub_area_id;
        for other_cell_id in self.filter_cell_id_in_given_sub_area_id(sub_area_id) {
            if other_cell_id != cell_id {
                let (other_cell_x_min, other_cell_x_max) = self.given_cell_x_border(other_cell_id);
                energy += overlap_energy(cell_x_min, cell_x_max, other_cell_x_min, other_cell_x_max);
            }
        }
        energy
    }

    pub fn given_cell_wire_energy(&self, cell_id : CellId) -> f64 {
        self.cell_static_dict.get(&cell_id).unwrap().wires.iter().map(|x| self.given_wire_energy(*x)).sum()
    }

    pub fn given_cell_wire_energy_with_new_pos(&self, cell_id : CellId, new_pos : &Pos) -> f64 {
        self.cell_static_dict.get(&cell_id).unwrap().wires.iter().map(|x| self.given_wire_energy_with_cell_new_pos(*x, cell_id, new_pos)).sum()
    }

    pub fn given_cell_x_energy(&self, cell_id : CellId, super_parameters : &SuperParameters) -> f64 {
        let mut energy = 0.;
        energy += self.given_cell_wire_energy(cell_id) * super_parameters.alpha_wire_energy;
        energy += self.given_cell_border_energy(cell_id) * super_parameters.alpha_border_energy;
        energy += self.given_cell_overlap_energy(cell_id) * super_parameters.alpha_overlap_energy;
        energy
    }

    pub fn given_cell_x_energy_with_new_pos(&self, cell_id : CellId, pos : &Pos, super_parameters : &SuperParameters) -> f64 {
        let mut energy = 0.;
        energy += self.given_cell_wire_energy_with_new_pos(cell_id, pos) * super_parameters.alpha_wire_energy;
        energy += self.given_cell_border_energy_with_new_pos(cell_id, pos) * super_parameters.alpha_border_energy;
        energy += self.given_cell_overlap_energy_with_new_pos(cell_id, pos) * super_parameters.alpha_overlap_energy;
        energy
    }

    pub fn given_cell_y_energy(&self, cell_id : CellId, super_parameters : &SuperParameters) -> f64 {
        let mut energy = 0.;
        energy += self.given_cell_wire_energy(cell_id) * super_parameters.alpha_wire_energy;
        energy += self.given_cell_border_energy(cell_id) * super_parameters.alpha_border_energy;
        energy += self.given_cell_overlap_energy(cell_id) * super_parameters.alpha_overlap_energy;
        energy
    }

    pub fn given_cell_y_energy_with_new_pos(&self, cell_id : CellId, pos : &Pos, super_parameters : &SuperParameters) -> f64 {
        let mut energy = 0.;
        energy += self.given_cell_wire_energy_with_new_pos(cell_id, pos) * super_parameters.alpha_wire_energy;
        energy += self.given_cell_border_energy_with_new_pos(cell_id, pos) * super_parameters.alpha_border_energy;
        energy += self.given_cell_overlap_energy_with_new_pos(cell_id, pos) * super_parameters.alpha_overlap_energy;
        energy
    }

    pub fn given_cell_x_energy_with_movement(&self, cell_id : CellId, disp : f64, super_parameters : &SuperParameters) -> f64 {
        let mut new_pos = self.cell_pos_dict.get(&cell_id).unwrap().clone();
        new_pos.impl_x_movement(disp);
        self.given_cell_x_energy_with_new_pos(cell_id, &new_pos, super_parameters)
    }

    pub fn all_wire_energy(&self) -> f64 {
        self.wire_static_dict.keys().map(|x| self.given_wire_energy(*x)).sum()
    }
}


#[test]
fn test_f64_min() {
    let a = [1.1, 2.2, 3.3];
    dbg!(a.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap());
    dbg!(a.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap());
}
use std::collections::BTreeSet;

use crate::adder_v2::floorplan_v::{AdderFPMain, CellId, CellStaticInfo, ModelParameters, Pos, SubAreaId, WireId, WireStaticInfo};

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
    overlap_len * overlap_len
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

    pub fn given_wire_energy(&self, wire_id : WireId) -> f64 {
        let (y_min, y_max, x_right_min, x_left_max) = self.given_wire_energy_inner(wire_id);
        let y_diff_abs = (y_max - y_min).abs();
        let x_diff_plus = (x_left_max - x_right_min).max(0.);
        y_diff_abs as f64 * self.model.y_scale + x_diff_plus as f64 * self.model.x_scale
    }

    pub fn given_sub_area_density_energy(&self, sub_area_id : SubAreaId) -> f64 {
        let cell_list = self.filter_cell_id_in_given_sub_area_id(sub_area_id);
        let width_sum = cell_list.iter().map(|x| self.given_cell_width(*x)).sum::<f64>();
        let sub_area_width = self.sub_area_dict.get(&sub_area_id).unwrap().x_len();
        let density = width_sum / sub_area_width;
        let density_energy = row_density_function(density);
        density_energy
    }

    pub fn given_cell_border_energy(&self, cell_id : CellId) -> f64 {
        let (cell_x_min, cell_x_max) = self.given_cell_x_border(cell_id);
        let sub_area_id = self.cell_pos_dict.get(&cell_id).unwrap().sub_area_id;
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

    pub fn given_cell_wire_energy(&self, cell_id : CellId) -> f64 {
        self.cell_static_dict.get(&cell_id).unwrap().wires.iter().map(|x| self.given_wire_energy(*x)).sum()
    }
}

// #[test]
// fn test_wire_energy_inner() {
//     let mut adder_fp_main = AdderFPMain::new(ModelParameters::for_76());
//     adder_fp_main.cell_pos_dict.insert(CellId(0), Pos {x: 10., y: 0, sub_area_index: 0});
//     adder_fp_main.cell_pos_dict.insert(CellId(1), Pos {x: 12., y: 4, sub_area_index: 0});
//     adder_fp_main.cell_pos_dict.insert(CellId(2), Pos {x: 10., y: 1, sub_area_index: 0});
//     adder_fp_main.cell_pos_dict.insert(CellId(3), Pos {x: 10., y: 2, sub_area_index: 0});

//     adder_fp_main.cell_static_dict.insert(CellId(0), CellStaticInfo {
//         name : "cell0".to_string(),
//         width : 6,
//         can_move : true,
//         wires : BTreeSet::new(),
//     });
//     adder_fp_main.cell_static_dict.insert(CellId(1), CellStaticInfo {
//         name : "cell1".to_string(),
//         width : 6,
//         can_move : true,
//         wires : BTreeSet::new(),
//     });
//     adder_fp_main.cell_static_dict.insert(CellId(2), CellStaticInfo {
//         name : "cell2".to_string(),
//         width : 6,
//         can_move : true,
//         wires : BTreeSet::new(),
//     });
//     adder_fp_main.cell_static_dict.insert(CellId(3), CellStaticInfo {
//         name : "cell3".to_string(),
//         width : 3,
//         can_move : true,
//         wires : BTreeSet::new(),
//     });

//     adder_fp_main.wire_static_dict.insert(WireId(0), WireStaticInfo {
//         name : "wire0".to_string(),
//         connected_cells : BTreeSet::from([CellId(0), CellId(1), CellId(2), CellId(3)]),
//     });

//     dbg!(adder_fp_main.given_wire_energy_inner(WireId(0)));
//     dbg!(adder_fp_main.given_wire_energy(WireId(0)));
// }

#[test]
fn test_f64_min() {
    let a = [1.1, 2.2, 3.3];
    dbg!(a.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap());
    dbg!(a.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap());
}
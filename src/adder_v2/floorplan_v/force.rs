use crate::adder_v2::floorplan_v::{AdderFPMain, CellId, SuperParameters};

const DELTA_X : f64 = 0.1;

impl AdderFPMain {
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
}
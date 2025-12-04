use crate::adder_v2::floorplan_v::{AdderFPMain, CellId, SubAreaId};

impl AdderFPMain {
    pub fn impl_cell_x_movement(&mut self, cell_id : CellId, disp : f64) {
        self.cell_pos_dict.get_mut(&cell_id).unwrap().x += disp;
    }

    pub fn can_cell_up_move(&self, cell_id : CellId) -> Option<SubAreaId> {
        let x = self.given_cell_x(cell_id);
        let y = self.given_cell_y(cell_id);
        for (sub_area_id, sub_area) in &self.sub_area_dict {
            if sub_area.y == y + 1 && sub_area.x_min < x && sub_area.x_max > x {
                return Some(*sub_area_id);
            }
        }
        None
    }

    pub fn can_cell_down_move(&self, cell_id : CellId) -> Option<SubAreaId> {
        let x = self.given_cell_x(cell_id);
        let y = self.given_cell_y(cell_id);
        for (sub_area_id, sub_area) in &self.sub_area_dict {
            if sub_area.y == y - 1 && sub_area.x_min < x && sub_area.x_max > x {
                return Some(*sub_area_id);
            }
        }
        None
    }

    pub fn impl_cell_up_move(&mut self, cell_id : CellId) {
        let sub_area_id = self.can_cell_up_move(cell_id).unwrap();
        self.cell_pos_dict.get_mut(&cell_id).unwrap().sub_area_id = sub_area_id;
    }

    pub fn impl_cell_down_move(&mut self, cell_id : CellId) {
        let sub_area_id = self.can_cell_down_move(cell_id).unwrap();
        self.cell_pos_dict.get_mut(&cell_id).unwrap().sub_area_id = sub_area_id;
    }
}
use std::collections::{BTreeMap, BTreeSet};

use crate::adder_v2::floorplan_v::{AdderFPMain, CellId, Pos, SubArea, SubAreaId, YMove};

impl Pos {
    pub fn impl_x_movement(&mut self, disp : f64) {
        self.x += disp;
    }

    pub fn impl_y_movement(&mut self, new_sub_area_id : SubAreaId) {
        self.sub_area_id = new_sub_area_id;
    }
}

impl YMove {
    pub fn all() -> Vec<YMove> {
        vec![YMove::Up, YMove::Down]
    }
}

impl AdderFPMain {
    pub fn impl_cell_x_movement(&mut self, cell_id : CellId, disp : f64) {
        self.cell_pos_dict.get_mut(&cell_id).unwrap().x += disp;
    }

    pub fn can_cell_y_move(&self, cell_id : CellId, y_move : YMove) -> Option<SubAreaId> {
        match y_move {
            YMove::Up => self.can_cell_up_move(cell_id),
            YMove::Down => self.can_cell_down_move(cell_id),
        }
    }

    fn can_cell_up_move(&self, cell_id : CellId) -> Option<SubAreaId> {
        let x = self.given_cell_x(cell_id);
        let y = self.given_cell_y(cell_id);
        for (sub_area_id, sub_area) in &self.sub_area_dict {
            if sub_area.y == y + 1 && sub_area.x_min < x && sub_area.x_max > x {
                return Some(*sub_area_id);
            }
        }
        None
    }

    fn can_cell_down_move(&self, cell_id : CellId) -> Option<SubAreaId> {
        let x = self.given_cell_x(cell_id);
        let y = self.given_cell_y(cell_id);
        for (sub_area_id, sub_area) in &self.sub_area_dict {
            if sub_area.y == y - 1 && sub_area.x_min < x && sub_area.x_max > x {
                return Some(*sub_area_id);
            }
        }
        None
    }

    pub fn impl_cell_y_movement(&mut self, cell_id : CellId, y_move : YMove) {
        match y_move {
            YMove::Up => self.impl_cell_up_move(cell_id),
            YMove::Down => self.impl_cell_down_move(cell_id),
        }
    }

    fn impl_cell_up_move(&mut self, cell_id : CellId) {
        let sub_area_id = self.can_cell_up_move(cell_id).unwrap();
        self.cell_pos_dict.get_mut(&cell_id).unwrap().sub_area_id = sub_area_id;
    }

    fn impl_cell_down_move(&mut self, cell_id : CellId) {
        let sub_area_id = self.can_cell_down_move(cell_id).unwrap();
        self.cell_pos_dict.get_mut(&cell_id).unwrap().sub_area_id = sub_area_id;
    }

    pub fn all_assert_interger(&mut self) {
        for cell_id in self.all_moveable_cell_ids() {
            let width = self.given_cell_width(cell_id);
            // 判断奇偶
            let is_odd = width.round() as i32 % 2 == 1;
            let x_now = self.given_cell_x(cell_id);
            let x_new = if is_odd {
                (x_now + 0.5).round() - 0.5
            } else {
                x_now.round()
            };
            self.impl_cell_x_movement(cell_id, x_new - x_now);
        }
    }

    pub fn all_remove_overlap(&mut self) {
        let mut cell_ids = self.all_moveable_cell_ids();

        

        let mut have_overlap = false;
        for i in 0..100 {
            let mut list = vec![];
            for cell_id in &cell_ids {
                list.push((self.given_cell_x_border(*cell_id).0, *cell_id));
            }
            list.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            

            for (_, cell_id) in &list {
                if self.given_cell_have_left_overlap(*cell_id)  {
                    self.cell_pos_dict.get_mut(cell_id).unwrap().x += 1.0;
                    have_overlap = true;
                }
            }
            if !have_overlap {
                return
            }
            have_overlap = false;
        }

        // panic!("all_remove_overlap failed");
    }
}
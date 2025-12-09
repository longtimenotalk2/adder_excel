use crate::adder_v2::floorplan_v::{AdderFPMain, CellId, Pos, SubArea, SubAreaId, WireId};

impl SubArea {
    pub fn x_len(&self) -> f64 {
        self.x_max - self.x_min
    }

    pub fn contains(&self, x: f64, y : i32) -> bool {
        self.x_min <= x && x <= self.x_max && self.y == y
    }
}

impl AdderFPMain {
    pub fn given_cell_x(&self, cell_id: CellId) -> f64 {
        match self.cell_static_dict.get(&cell_id).unwrap().can_move {
            true => {
                self.cell_pos_dict.get(&cell_id).unwrap().x
            },
            false => {
                self.cell_fixed_pos_dict.get(&cell_id).unwrap().0
            }
        }
    }

    pub fn given_cell_sub_area_id(&self, cell_id: CellId) -> SubAreaId {
        self.cell_pos_dict.get(&cell_id).unwrap().sub_area_id
    }

    pub fn given_cell_x_border(&self, cell_id: CellId) -> (f64, f64) {
        let cell_width = self.cell_static_dict.get(&cell_id).unwrap().width as f64;
        let x_middle = match self.cell_static_dict.get(&cell_id).unwrap().can_move {
            true => {
                self.cell_pos_dict.get(&cell_id).unwrap().x
            },
            false => {
                self.cell_fixed_pos_dict.get(&cell_id).unwrap().0
            }
        };
        (x_middle - cell_width / 2.0, x_middle + cell_width / 2.0)
    }

    pub fn given_cell_x_border_with_cell_new_pos(&self, cell_id: CellId, cell_id_changed: CellId, cell_new_pos: &Pos) -> (f64, f64) {
        let cell_width = self.cell_static_dict.get(&cell_id).unwrap().width as f64;
        let x_middle = if cell_id == cell_id_changed {
            cell_new_pos.x
        } else {
            match self.cell_static_dict.get(&cell_id).unwrap().can_move {
                true => {
                    self.cell_pos_dict.get(&cell_id).unwrap().x
                },
                false => {
                    self.cell_fixed_pos_dict.get(&cell_id).unwrap().0
                }
            }
        };
        
        (x_middle - cell_width / 2.0, x_middle + cell_width / 2.0)
    }

    pub fn given_sub_area_x_border(&self, sub_area_id: SubAreaId) -> (f64, f64) {
        let sub_area = self.sub_area_dict.get(&sub_area_id).unwrap();
        (sub_area.x_min, sub_area.x_max)
    }

    pub fn given_sub_area_density(&self, sub_area_id: SubAreaId) -> f64 {
        let cell_list = self.filter_cell_id_in_given_sub_area_id(sub_area_id);
        let width_sum = cell_list.iter().map(|x| self.given_cell_width(*x)).sum::<f64>();
        let sub_area_width = self.sub_area_dict.get(&sub_area_id).unwrap().x_len();
        let density = width_sum / sub_area_width;
        density
    }

    pub fn given_cell_y(&self, cell_id: CellId) -> i32 {
        match self.cell_static_dict.get(&cell_id).unwrap().can_move {
            true => {
                let sub_area_id = self.cell_pos_dict.get(&cell_id).unwrap().sub_area_id;
                let y = self.sub_area_dict.get(&sub_area_id).unwrap().y;
                y
            },
            false => {
                self.cell_fixed_pos_dict.get(&cell_id).expect(&format!("can not find pos for fixed cell id {}", cell_id.0)).1
            }
        }
    }

    pub fn given_cell_y_with_cell_new_pos(&self, cell_id: CellId, cell_id_changed: CellId, cell_new_pos: &Pos) -> i32 {
        if cell_id == cell_id_changed {
            let sub_area_id = cell_new_pos.sub_area_id;
            let y = self.sub_area_dict.get(&sub_area_id).unwrap().y;
            return y
        }
        match self.cell_static_dict.get(&cell_id).unwrap().can_move {
            true => {
                let sub_area_id = self.cell_pos_dict.get(&cell_id).unwrap().sub_area_id;
                let y = self.sub_area_dict.get(&sub_area_id).unwrap().y;
                y
            },
            false => {
                self.cell_fixed_pos_dict.get(&cell_id).expect(&format!("can not find pos for fixed cell id {}", cell_id.0)).1
            }
        }
    }

    pub fn given_cell_width(&self, cell_id: CellId) -> f64 {
        self.cell_static_dict.get(&cell_id).unwrap().width
    }

    pub fn get_sub_area_id(&self, x : f64, y : i32) -> SubAreaId {
        for (id, sub_area) in self.sub_area_dict.iter() {
            if sub_area.contains(x, y) {
                return *id;
            }
        }
        panic!("No sub area found for x: {}, y: {}", x, y);
    }

    pub fn get_wire_id_by_name(&self, name: &str) -> WireId {
        for (wire_id, wire) in self.wire_static_dict.iter() {
            if wire.name == name {
                return *wire_id;
            }
        }
        panic!("No wire found for name: {}", name);
    }

    pub fn get_wire_id_by_name_may(&self, name: &str) -> Option<WireId> {
        for (wire_id, wire) in self.wire_static_dict.iter() {
            if wire.name == name {
                return Some(*wire_id);
            }
        }
        None
    }

    pub fn get_cell_id_by_name(&self, name: &str) -> CellId {
        for (cell_id, cell) in self.cell_static_dict.iter() {
            if cell.name == name {
                return *cell_id;
            }
        }
        panic!("No cell found for name: {}", name);
    }

    // pub fn all_cell_ids(&self) -> Vec<CellId> {
    //     self.cell_static_dict.keys().cloned().collect()
    // }

    pub fn all_moveable_cell_ids(&self) -> Vec<CellId> {
        self.cell_static_dict.iter().filter(|(_, cell)| cell.can_move).map(|(id, _)| *id).collect()
    }
}
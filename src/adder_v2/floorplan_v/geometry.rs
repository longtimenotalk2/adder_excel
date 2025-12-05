use crate::adder_v2::floorplan_v::{AdderFPMain, CellId, SubArea, SubAreaId, WireId};

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
        self.cell_pos_dict.get(&cell_id).unwrap().x
    }

    pub fn given_cell_sub_area_id(&self, cell_id: CellId) -> SubAreaId {
        self.cell_pos_dict.get(&cell_id).unwrap().sub_area_id
    }

    pub fn given_cell_x_border(&self, cell_id: CellId) -> (f64, f64) {
        let x_middle = self.cell_pos_dict.get(&cell_id).unwrap().x;
        let cell_width = self.cell_static_dict.get(&cell_id).unwrap().width as f64;
        (x_middle - cell_width / 2.0, x_middle + cell_width / 2.0)
    }

    pub fn given_sub_area_x_border(&self, sub_area_id: SubAreaId) -> (f64, f64) {
        let sub_area = self.sub_area_dict.get(&sub_area_id).unwrap();
        (sub_area.x_min, sub_area.x_max)
    }

    pub fn given_cell_y(&self, cell_id: CellId) -> i32 {
        let sub_area_id = self.cell_pos_dict.get(&cell_id).unwrap().sub_area_id;
        let y = self.sub_area_dict.get(&sub_area_id).unwrap().y;
        y
    }

    pub fn given_cell_width(&self, cell_id: CellId) -> i32 {
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
}
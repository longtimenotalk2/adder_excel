use crate::adder_v2::floorplan_v::{AdderFPMain, CellId, SubAreaId};

impl AdderFPMain {
    pub fn filter_cell_id_in_given_sub_area_id(&self, sub_area_id: SubAreaId) -> Vec<CellId> {
        let mut ret = vec![];
        for (cell_id, pos) in &self.cell_pos_dict {
            if pos.sub_area_id == sub_area_id {
                ret.push(*cell_id);
            }
        }
        ret
    }
}
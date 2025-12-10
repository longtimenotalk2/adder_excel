use crate::adder_v2::floorplan_m_v1::{CellId, FloorPlanMV1};

impl FloorPlanMV1 {
    pub fn find_cell_id_from_name(&self, name: &str) -> Option<CellId> {
        for (i, cell) in &self.cell_static_data {
            if cell.name == name {
                return Some(*i);
            }
        }
        None
    }
}
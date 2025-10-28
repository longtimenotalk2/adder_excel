use crate::adder_v2::cell::cell_info::{CellInfo, SpecialInfo};

impl CellInfo {
    pub fn is_power_vddh(&self) -> bool {
        self.special_infos.contains(&SpecialInfo("VDH".to_string()))
    }
}
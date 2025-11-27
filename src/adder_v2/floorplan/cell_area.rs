use crate::adder_v2::cell::cell_body::CellBody;

impl CellBody {
    pub fn width(&self) -> i32 {
        self.mos_num() / 2 + 1
    }
}
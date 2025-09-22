use std::collections::BTreeMap;

use crate::std::logic_block::{LogicBlock, Port};

impl LogicBlock {
    // capi 是指以CMOS计数的电容，1个mos管为1。
    pub fn capi_input(&self) -> BTreeMap<Port, i32> {
        let mut ret = BTreeMap::new();

        for port in self.ports_input() {
            let cap = match self {
                LogicBlock::XNR2 | LogicBlock::XOR2 | LogicBlock::XNR2DOUT | LogicBlock::XOR2DOUT => 4,
                _ => 2,
            };
            ret.insert(port, cap);
        }

        ret
    }

    pub fn capi_output(&self) -> BTreeMap<Port, i32> {
        let mut ret = BTreeMap::new();

        for port in self.ports_output() {
            let cap = if port == Port("O1".to_string()) {
                2
            } else {
                0
            };
            ret.insert(port, cap);
        }

        ret
    }
}
use std::collections::BTreeMap;

use crate::std::{adder::Adder, logic_block::Port};

impl Adder {
    // 按照cell顺序，给出每个cell的各输出pin的cap
    pub fn capi_calc(&self) -> Vec<BTreeMap<Port, i32>> {
        let mut ret = vec![];

        for cell in &self.cells {
            let mut inner = BTreeMap::new();
            for (port_output, wire_output) in &cell.logic_block_map.outputs {
                let mut cap = *cell.capi_output().get(&port_output).unwrap();
                for cell_another in &self.cells {
                    for (port_input, wire_input) in &cell_another.logic_block_map.inputs {
                        if wire_input == wire_output {
                            cap += *cell_another.capi_input().get(&port_input).unwrap();
                        }
                    }
                }
                inner.insert(port_output.clone(), cap);
            }
            ret.push(inner);
        }

        assert_eq!(ret.len(), self.cells.len());

        ret
    }
}
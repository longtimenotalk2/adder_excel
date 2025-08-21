use std::collections::{BTreeMap, BTreeSet};

use crate::std::logic_block::Port;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CustomLogicBlock {
    NR4,
    NR6,
    AOI221,
    AOI2221,
}

impl CustomLogicBlock {
    pub fn ports_input_raw(&self) -> Vec<&'static str> {
        let ports: Vec<&'static str> = match self {
            Self::NR4 => vec!["A1", "A2", "A3", "A4"],
            Self::NR6 => vec!["A1", "A2", "A3", "A4", "A5", "A6"],
            Self::AOI221 => vec!["A1", "A2", "B1", "B2", "C"],
            Self::AOI2221 => vec!["A1", "A2", "B1", "B2", "C1", "C2", "D"],
        };
        ports
    }

    pub fn ports_input(&self) -> BTreeSet<Port> {
        self.ports_input_raw().iter().map(|p| Port(p.to_string())).collect()
    }

    pub fn ports_output_raw(&self) -> Vec<&'static str> {
        vec!["ZN"]
    }

    pub fn ports_output(&self) -> BTreeSet<Port> {
        self.ports_output_raw().iter().map(|p| Port(p.to_string())).collect()
    }

    pub fn calc(&self, inputs : BTreeMap<Port, bool>) -> BTreeMap<Port, bool> {
        assert_eq!(inputs.keys().cloned().collect::<BTreeSet<_>>(), self.ports_input());
        match self {
            Self::NR4 => {
                let a1 = *inputs.get(&Port::new("A1")).unwrap();
                let a2 = *inputs.get(&Port::new("A2")).unwrap();
                let a3 = *inputs.get(&Port::new("A3")).unwrap();
                let a4 = *inputs.get(&Port::new("A4")).unwrap();
                let zn = !(a1 || a2 || a3 || a4);
                BTreeMap::from([(Port::new("ZN"), zn)])
            },
            Self::NR6 => {
                let a1 = *inputs.get(&Port::new("A1")).unwrap();
                let a2 = *inputs.get(&Port::new("A2")).unwrap();
                let a3 = *inputs.get(&Port::new("A3")).unwrap();
                let a4 = *inputs.get(&Port::new("A4")).unwrap();
                let a5 = *inputs.get(&Port::new("A5")).unwrap();
                let a6 = *inputs.get(&Port::new("A6")).unwrap();
                let zn = !(a1 || a2 || a3 || a4 || a5 || a6);
                BTreeMap::from([(Port::new("ZN"), zn)])
            },
            Self::AOI221 => {
                let a1 = *inputs.get(&Port::new("A1")).unwrap();
                let a2 = *inputs.get(&Port::new("A2")).unwrap();
                let b1 = *inputs.get(&Port::new("B1")).unwrap();
                let b2 = *inputs.get(&Port::new("B2")).unwrap();
                let c = *inputs.get(&Port::new("C")).unwrap();
                let zn = !((a1 && a2) || (b1 && b2) || c);
                BTreeMap::from([(Port::new("ZN"), zn)])
            },
            Self::AOI2221 => {
                let a1 = *inputs.get(&Port::new("A1")).unwrap();
                let a2 = *inputs.get(&Port::new("A2")).unwrap();
                let b1 = *inputs.get(&Port::new("B1")).unwrap();
                let b2 = *inputs.get(&Port::new("B2")).unwrap();
                let c1 = *inputs.get(&Port::new("C1")).unwrap();
                let c2 = *inputs.get(&Port::new("C2")).unwrap();
                let d = *inputs.get(&Port::new("D")).unwrap();
                let zn = !((a1 && a2) || (b1 && b2) || (c1 && c2) || d);
                BTreeMap::from([(Port::new("ZN"), zn)])
            }
        }
    }
}
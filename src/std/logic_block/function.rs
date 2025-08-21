use std::collections::{BTreeMap, BTreeSet};

use crate::std::logic_block::{LogicBlock, Port};

impl LogicBlock {
    pub fn calc(&self, inputs : BTreeMap<Port, bool>) -> BTreeMap<Port, bool> {
        assert_eq!(inputs.keys().cloned().collect::<BTreeSet<_>>(), self.ports_input());

        match self {
            LogicBlock::Custom(custom) => custom.calc(inputs),
            LogicBlock::XOR2DOUT => {
                let a1 = *inputs.get(&Port::new("A1")).unwrap();
                let a2 = *inputs.get(&Port::new("A2")).unwrap();
                let o1 = !(a1 || a2);
                let z = a1 ^ a2;
                BTreeMap::from([(Port::new("O1"), o1), (Port::new("Z"), z)])
            }
            LogicBlock::XNR2DOUT => {
                let a1 = *inputs.get(&Port::new("A1")).unwrap();
                let a2 = *inputs.get(&Port::new("A2")).unwrap();
                let o1 = !(a1 && a2);
                let zn = !a1 ^ a2;
                BTreeMap::from([(Port::new("O1"), o1), (Port::new("ZN"), zn)])
            }
            _ => {
                let out = match self {
                    LogicBlock::INV => {
                        let i = *inputs.get(&Port::new("I")).unwrap();
                        !i
                    },
                    LogicBlock::ND2 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        !(a1 && a2)
                    },
                    LogicBlock::NR2 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        !(a1 || a2)
                    },
                    LogicBlock::AN2 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        a1 && a2
                    },
                    LogicBlock::OR2 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        a1 || a2
                    },
                    LogicBlock::IND2 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let b1 = *inputs.get(&Port::new("B1")).unwrap();
                        !(!a1 && b1)
                    },
                    LogicBlock::INR2 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let b1 = *inputs.get(&Port::new("B1")).unwrap();
                        !(!a1 || b1)
                    },
                    LogicBlock::XOR2 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        a1 ^ a2
                    },
                    LogicBlock::XNR2 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        !a1 ^ a2
                    },
                    LogicBlock::XOR2DOUT => unimplemented!(),
                    LogicBlock::XNR2DOUT => unimplemented!(),
                    LogicBlock::AOI21 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        !((a1 && a2) || b)
                    },
                    LogicBlock::OAI21 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        !((a1 || a2) && b)
                    },
                    LogicBlock::AO21 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        (a1 && a2) || b
                    },
                    LogicBlock::OA21 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        (a1 || a2) && b
                    },
                    LogicBlock::IAOI21 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        !((a1 && !a2) || b)
                    },
                    LogicBlock::IOAI21 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        !((a1 || !a2) && b)
                    },
                    LogicBlock::AOI22 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b1 = *inputs.get(&Port::new("B1")).unwrap();
                        let b2 = *inputs.get(&Port::new("B2")).unwrap();
                        !((a1 && a2) || (b1 && b2))
                    },
                    LogicBlock::OAI22 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b1 = *inputs.get(&Port::new("B1")).unwrap();
                        let b2 = *inputs.get(&Port::new("B2")).unwrap();
                        !((a1 || a2) && (b1 || b2))
                    },
                    LogicBlock::AOAI211 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        let c = *inputs.get(&Port::new("C")).unwrap();
                        !(((a1 && a2) || b) && c)
                    },
                    LogicBlock::OAOI211 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        let c = *inputs.get(&Port::new("C")).unwrap();
                        !(((a1 || a2) && b) || c)
                    },
                    LogicBlock::AOA211 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        let c = *inputs.get(&Port::new("C")).unwrap();
                        ((a1 && a2) || b) && c
                    },
                    LogicBlock::OAO211 => {
                        let a1 = *inputs.get(&Port::new("A1")).unwrap();
                        let a2 = *inputs.get(&Port::new("A2")).unwrap();
                        let b = *inputs.get(&Port::new("B")).unwrap();
                        let c = *inputs.get(&Port::new("C")).unwrap();
                        ((a1 || a2) && b) || c
                    },
                    LogicBlock::Custom(..) => unimplemented!()
                };
                BTreeMap::from([(self.ports_output().first().unwrap().clone(), out)])
            }
        }

        
    }
}
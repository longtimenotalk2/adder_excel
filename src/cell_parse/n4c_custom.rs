use crate::{cell_parse::RealCell, std::{adder::{CustomDemand, Drive}, logic_block::LogicBlock}};

impl RealCell {
    pub fn parse_n4c_custom(
        logic_block : LogicBlock,
        drive : Drive,
        custom_demand : Vec<CustomDemand>,
    ) -> Self {
        assert_eq!(custom_demand.len(), 1);
        match &custom_demand[0] {
            CustomDemand::Domino(domino) => {
                todo!()
            },
            _ => unimplemented!()
        }
    }
}
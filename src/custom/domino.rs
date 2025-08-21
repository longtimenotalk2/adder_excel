use crate::{custom::custom_logic_block::CustomLogicBlock, std::{logic_block::LogicBlock, node_create::{create_middle_node::WireManager, LogiBlockHint, LogicBlockCreateError, LogicBlockMappingTable}, wire::{Flag, Wire}}};

#[derive(Debug, Clone)]
pub enum DominoPolar {
    P,
    N,
}

#[derive(Debug, Clone)]
pub struct DominoDemand {
    pub logic_block : LogicBlock,
    pub polar : DominoPolar,
    pub is_start : bool,
}

impl DominoDemand {
    pub fn from_strings(ss : &[String]) -> DominoDemand {
        assert_eq!(ss[0], "DOM");
        let logic_block = match ss[1].as_str() {
            "AOI21" => LogicBlock::AOI21,
            "OAI21" => LogicBlock::OAI21,
            "NR2" => LogicBlock::NR2,
            "NR4" => LogicBlock::Custom(CustomLogicBlock::NR4),
            "NR6" => LogicBlock::Custom(CustomLogicBlock::NR6),
            "AOI221" => LogicBlock::Custom(CustomLogicBlock::AOI221),
            "AOI2221" => LogicBlock::Custom(CustomLogicBlock::AOI2221),
            _ => panic!("Unknown logic block {}", ss[1]),
        };
        let polar = match ss[2].as_str() {
            "P" => DominoPolar::P,
            "N" => DominoPolar::N,
            _ => panic!("Unknown polar {}", ss[2]),
        };
        let is_start = if let Some(start_note) = ss.get(3) {
            assert_eq!(start_note, "S");
            true
        } else {
            false
        };
        DominoDemand {
            logic_block,
            polar,
            is_start,
        }
    }

    pub fn create_logic_block_mapping_table(
        &self, 
        target_wire : &Wire,
        manager : &mut WireManager,
        hint : &LogiBlockHint,
    ) -> Result<LogicBlockMappingTable, LogicBlockCreateError> {
        match hint {
            LogiBlockHint::Normal { flags, is_out_inv, custom_input_invs, custom_input_lens } => {
                // if flags.len() == 1 {
                //     match flags {
                //         &vec![Flag::P; 4] => {
                //             todo!()
                //         }
                //         _ => {
                //             todo!()
                //         }
                //     }
                // } else {
                //     todo!()
                // }
                todo!()
            }
            _ => unimplemented!()
        }
    }
}
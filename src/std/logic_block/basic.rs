use crate::std::logic_block::LogicBlock;

impl LogicBlock {
    pub fn rev(&self) -> Self {
        match self {
            LogicBlock::INV => LogicBlock::INV,
            LogicBlock::ND2 => LogicBlock::NR2,
            LogicBlock::NR2 => LogicBlock::NR2,
            LogicBlock::AN2 => LogicBlock::OR2,
            LogicBlock::OR2 => LogicBlock::AN2,
            LogicBlock::IND2 => LogicBlock::INR2,
            LogicBlock::INR2 => LogicBlock::IND2,
            LogicBlock::XOR2 => LogicBlock::XNR2,
            LogicBlock::XNR2 => LogicBlock::XOR2,
            LogicBlock::XOR2DOUT => LogicBlock::XNR2DOUT,
            LogicBlock::XNR2DOUT => LogicBlock::XOR2DOUT,
            LogicBlock::AOI21 => LogicBlock::OAI21,
            LogicBlock::OAI21 => LogicBlock::AOI21,
            LogicBlock::AO21 => LogicBlock::OA21,
            LogicBlock::OA21 => LogicBlock::AO21,
            LogicBlock::IAOI21 => LogicBlock::IOAI21,
            LogicBlock::IOAI21 => LogicBlock::IAOI21,
            LogicBlock::AOI22 => LogicBlock::OAI22,
            LogicBlock::OAI22 => LogicBlock::AOI22,
            LogicBlock::AOAI211 => LogicBlock::OAOI211,
            LogicBlock::OAOI211 => LogicBlock::AOAI211,
            LogicBlock::AOA211 => LogicBlock::OAO211,
            LogicBlock::OAO211 => LogicBlock::AOA211,
        }
    }
}
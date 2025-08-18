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

    pub fn if_rev(self, is_rev: bool) -> Self {
        if is_rev {
            self.rev()
        } else {
            self
        }
    }

    pub fn if_add_out_inv(self, is_out_inv: bool) -> Self {
        if is_out_inv {
            match self {
                LogicBlock::INV => unimplemented!(),
                LogicBlock::ND2 => LogicBlock::AN2,
                LogicBlock::NR2 => LogicBlock::OR2,
                LogicBlock::AN2 => unimplemented!(),
                LogicBlock::OR2 => unimplemented!(),
                LogicBlock::IND2 => unimplemented!(),
                LogicBlock::INR2 => unimplemented!(),
                LogicBlock::XOR2 => unimplemented!(),
                LogicBlock::XNR2 => unimplemented!(),
                LogicBlock::XOR2DOUT => unimplemented!(),
                LogicBlock::XNR2DOUT => unimplemented!(),
                LogicBlock::AOI21 => LogicBlock::AO21,
                LogicBlock::OAI21 => LogicBlock::OA21,
                LogicBlock::AO21 => unimplemented!(),
                LogicBlock::OA21 => unimplemented!(),
                LogicBlock::IAOI21 => unimplemented!(),
                LogicBlock::IOAI21 => unimplemented!(),
                LogicBlock::AOI22 => unimplemented!(),
                LogicBlock::OAI22 => unimplemented!(),
                LogicBlock::AOAI211 => LogicBlock::AOA211,
                LogicBlock::OAOI211 => LogicBlock::OAO211,
                LogicBlock::AOA211 => unimplemented!(),
                LogicBlock::OAO211 => unimplemented!(),
            }
        } else {
            self
        }
    }

    pub fn aoi21_like_input_inv(self) -> Self {
        match self {
            LogicBlock::AOI21 => LogicBlock::IAOI21,
            LogicBlock::OAI21 => LogicBlock::IOAI21,
            _ => unimplemented!()
        }
    }
}
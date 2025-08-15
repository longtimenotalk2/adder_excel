use crate::std::wire::Wire;

impl Wire {
    pub fn rev(&self) -> Self {
        let mut ret = self.clone();
        ret.is_neg = !ret.is_neg;
        ret
    }
}
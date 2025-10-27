
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Drive {
    D1,
    D2,
}

impl Default for Drive {
    fn default() -> Self {
        Drive::D1
    }
}

#[derive(Debug, Clone, Default)]
pub struct CellInfo {
    pub drive : Drive
}

impl CellInfo {
    pub fn default() -> Self {
        Self {
            drive : Drive::D1,
        }
    }

    pub fn to_string(&self) -> String {
        let mut txt = String::new();
        if self.drive == Drive::D2 {
            txt.push_str("D2");
        }
        txt
    }
}
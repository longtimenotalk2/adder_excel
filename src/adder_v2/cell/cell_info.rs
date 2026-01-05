use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpecialInfo(pub String);

impl SpecialInfo {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Drive {
    D1,
    D2,
    D3,
    D4,
}

impl Default for Drive {
    fn default() -> Self {
        Drive::D1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CellInfo {
    pub drive : Drive,
    pub special_infos: BTreeSet<SpecialInfo>,
    pub notation : Option<usize>,
}

impl CellInfo {
    pub fn default() -> Self {
        Self {
            drive : Drive::D1,
            special_infos: BTreeSet::new(),
            notation : None,
        }
    }

    pub fn to_string(&self) -> String {
        let mut tokens = vec![];
        if self.drive == Drive::D2 {
            tokens.push("D2".to_string());
        }
        for special in &self.special_infos {
            tokens.push(special.0.clone());
        }
        tokens.join(", ")
    }

    pub fn is_default(&self) -> bool {
        self.special_infos.is_empty()
    }
}

#[test]
fn test_cell_info() {
    let mut info = CellInfo {
        drive : Drive::D2,
        special_infos: BTreeSet::new(),
        notation : None,
    };
    info.special_infos.insert(SpecialInfo("L2H".to_string()));
    dbg!(info.to_string());
}
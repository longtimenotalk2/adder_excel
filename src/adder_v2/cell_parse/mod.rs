pub mod n3e;

pub struct ReadCellName(pub String);

pub enum ReadCellType {
    Std,
    Custom,
    Lhw,
}

pub enum Process {
    N3E,
}
pub mod project;
pub mod io;

use std::collections::{BTreeMap, BTreeSet};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CellId(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct WireId(u8);

#[derive(Debug, Clone)]
struct CellPos {
    x : i32,
    y : i32,
}

#[derive(Debug)]
struct CellStaticData {
    name : String,
    width : i32,
    connected_wire_set : BTreeSet<WireId>
}

#[derive(Debug)]
struct WireStaticData {
    name : String,
    connected_cell_set : BTreeSet<CellId>
}

#[derive(Debug)]
struct FloorPlanMV1 {
    cell_static_data : BTreeMap<CellId, CellStaticData>,
    wire_static_data : BTreeMap<WireId, WireStaticData>,
    cell_pos : BTreeMap<CellId, CellPos>,
}

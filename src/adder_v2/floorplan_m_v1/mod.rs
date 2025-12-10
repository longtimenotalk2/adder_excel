pub mod project;
pub mod io;
pub mod init;
pub mod basic;
pub mod draw;

use std::collections::{BTreeMap, BTreeSet};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CellId(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct WireId(u16);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct M1YRange {
    start : i32,
    end : i32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct M1XEnum(BTreeSet<i32>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct M1Certain {
    x : i32,
    y : M1YRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum FA1NCate{
    W,
    CONBUF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum FA1NPose {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct FA1NInfo {
    cate : FA1NCate,
    pose : FA1NPose,
}

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

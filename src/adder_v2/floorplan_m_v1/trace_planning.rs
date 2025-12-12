use std::collections::{BTreeMap, BTreeSet};

use crate::adder_v2::floorplan_m_v1::{CellId, WireId};

struct M1Track {
    x : i32,
    y_range : (i32, i32),
}

struct M1XSelectedRange {
    x_min : i32,
    x_max : i32,
}

struct WireM1Manager {
    finish : Vec<M1Track>,
    unfinished_blocks : Vec<M1XSelectedRange>,
}

struct M0DirectData {
    data : BTreeMap<WireId, Vec<BTreeSet<CellId>>>
}
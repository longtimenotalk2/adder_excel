pub mod energy;
pub mod geometry;
pub mod filter;
pub mod force;
pub mod movement;
mod init;

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CellId (u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct WireId (u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SubAreaId (u16);

#[derive(Debug, Clone)]
struct Pos {
    x : f64,
    sub_area_id : SubAreaId,
}

/// SubArea是一段，y确定，x是一段范围的区域。
#[derive(Debug, Clone)]
struct SubArea {
    y : i32,
    sub_area_index : usize,
    x_min : f64,
    x_max : f64,
}

#[derive(Debug, Clone)]
struct CellStaticInfo {
    name : String,
    width : i32,
    can_move : bool,
    wires : BTreeSet<WireId>,
}

#[derive(Debug, Clone)]
struct WireStaticInfo {
    name : String,
    connected_cells : BTreeSet<CellId>,
}

#[derive(Debug, Clone)]
struct AdderFPMain {
    sub_area_dict : BTreeMap<SubAreaId, SubArea>,
    cell_static_dict : BTreeMap<CellId, CellStaticInfo>,
    wire_static_dict : BTreeMap<WireId, WireStaticInfo>,
    cell_pos_dict : BTreeMap<CellId, Pos>,
    model : ModelParameters,
}

#[derive(Debug, Clone)]
struct ModelParameters {
    x_scale : f64,
    y_scale : f64,
}

impl AdderFPMain {
    fn new(model : ModelParameters) -> Self {
        Self {
            cell_pos_dict : BTreeMap::new(),
            cell_static_dict : BTreeMap::new(),
            wire_static_dict : BTreeMap::new(),
            model,
            sub_area_dict : BTreeMap::new(),
        }
    }
}

impl ModelParameters {
    fn for_76() -> Self {
        Self {
            x_scale : 48.,
            y_scale : 156.,
        }
    }
}

struct SuperParameters {
    alpha_wire_energy : f64,
    alpha_density_energy : f64,
    alpha_border_energy : f64,
    alpha_overlap_energy : f64,
}
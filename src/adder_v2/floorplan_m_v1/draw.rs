use svg::{Node, node::element::SVG};

use crate::adder_v2::floorplan_m_v1::FloorPlanMV1;


fn draw_addition(mut document : SVG, to_be_draw : Vec<Box<dyn Node>>) -> SVG {
    for item in to_be_draw {
        document = document.add(item)
    }
    document
}

pub(super) enum DrawWhat {
    Cell,
}

impl FloorPlanMV1 {
    pub fn draw(&self, draw_what : Vec<DrawWhat>, path : &str, x_max : i32, y_max : i32) {
        
    }
}
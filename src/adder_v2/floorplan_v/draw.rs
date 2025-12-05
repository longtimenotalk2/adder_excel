use svg::{Document, node::element::Rectangle};

use crate::adder_v2::floorplan_v::AdderFPMain;

#[derive(Debug, Clone)]
pub struct Art {
    x_len : usize, 
    y_len : usize, 
    x_px : f32, 
    y_px : f32,
}

const BORDER : f32 = 100. ;

const ART : Art = Art {
    x_len : 192,
    y_len : 7,
    x_px : 48.,
    y_px : 156.,
};

impl AdderFPMain {
    pub fn draw_default_art(&self) {
        let art = ART;
        let mut document = Document::new().set("viewBox", (0, 0, 
                    BORDER*2. + art.x_len as f32 * art.x_px, 
                    BORDER*2. + art.y_len as f32 * art.y_px, 
                )); // 视口大小

        // 创建白色背景矩形
        document = document.add(
        Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", "white")
        );

        // 创建主要区域的黑色矩形
        document = document.add(
        Rectangle::new()
            .set("x", BORDER)
            .set("y", BORDER)
            .set("width", art.x_len as f32 * art.x_px)
            .set("height", art.y_len as f32 * art.y_px)
            .set("fill", "white")
            .set("stroke", "black")
            .set("stroke-width", 1)
        );

        svg::save("place.svg", &document).unwrap();
    }
}
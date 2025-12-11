use svg::{Document, Node, node::element::{Line, Rectangle, SVG, Text}};

use crate::adder_v2::{adder::{self, Adder}, floorplan_m_v1::FloorPlanMV1};


// fn draw_addition(mut document : SVG, to_be_draw : Vec<Box<dyn Node>>) -> SVG {
//     for item in to_be_draw {
//         document = document.add(item)
//     }
//     document
// }

pub(super) enum DrawWhat<'a> {
    Cell(&'a Adder),
}

pub(super) const ART : Art = Art {
    x_px : 48.,
    y_px : 156.,
};

const BORDER : f32 = 400. ;

#[derive(Debug, Clone)]
pub(crate) struct Art {
    x_px : f32, 
    y_px : f32,
}

impl FloorPlanMV1 {
    fn draw_cell(&self, mut document : SVG, y_len : i32, art : &Art, adder : &Adder) -> SVG {

        // 绘制所有cell
        for (cell_id, cell_pos) in &self.cell_pos {
            let color = if let Some(adder_cell) = adder.cells.get(cell_id.0 as usize) {
                adder_cell.1.node.logic.color_hex_inner()
            } else {
                "red"
            };
            let cell_info = self.cell_static_data.get(cell_id).unwrap();
            let x = cell_pos.x as f32;
            let y = cell_pos.y as f32;
            let width = cell_info.width as f32;
            

            let x_given = BORDER + x * art.x_px;
            let y_given = BORDER + (y_len as f32 - y - 1.) * art.y_px;
            let width_given = width * art.x_px;
            let height_given = art.y_px;

            // cell rec main
            document = document.add(Rectangle::new()
                .set("x", x_given)
                .set("y", y_given)
                .set("width", width_given)
                .set("height", height_given)
                .set("fill", color)
                .set("stroke", "black")
                .set("stroke-width", 1)
                .set("opacity", 0.8)
            );

            let x_given = BORDER + (x + width / 2.) * art.x_px;
            let y_given = BORDER + (y_len as f32 - y -0.5) * art.y_px;

            document = document.add(Text::new(&format!("{}", cell_info.name.as_str()))
                .set("x", x_given)
                .set("y", y_given-40.)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", 20.)
            );
            let wire_num = cell_info.connected_wire_set.len();
            
            document = document.add(Text::new(&format!("{wire_num}"))
                .set("x", x_given)
                .set("y", y_given+5.)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", 50.)
            );

        }

        document
    }


    pub fn draw(&self, draw_what : Vec<DrawWhat>, path : &str, x_len : i32, y_len : i32, art : &Art) {
        let mut document = Document::new().set("viewBox", (0, 0, 
                    BORDER*2. + x_len as f32 * art.x_px, 
                    BORDER*2. + y_len as f32 * art.y_px, 
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

        // 画纵向网格线
        for x in 1..x_len - 1 {
            document = document.add(
            Line::new()
                    .set("x1", x as f32 * art.x_px + BORDER)
                    .set("y1", BORDER)
                    .set("x2", x as f32 * art.x_px + BORDER)
                    .set("y2", BORDER + y_len as f32 * art.y_px)
                    .set("stroke", "black")
                    .set("stroke-dasharray", "8, 16")
            );
        }

        // 主要绘制
        for draw in draw_what {
            match draw {
                DrawWhat::Cell(adder) => document = self.draw_cell(document, y_len, art, adder),
            }
        }

        svg::save(&format!("{path}.svg"), &document).unwrap();
    }
}
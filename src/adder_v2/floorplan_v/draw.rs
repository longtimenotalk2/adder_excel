use svg::{Document, node::element::{Line, Rectangle, Text}};

use crate::adder_v2::{adder::Adder, floorplan_v::{AdderFPMain, YMove}};

#[derive(Debug, Clone)]
pub struct Art {
    x_len : usize, 
    y_len : usize, 
    x_px : f32, 
    y_px : f32,
}

const BORDER : f32 = 400. ;

const ART : Art = Art {
    x_len : 192,
    y_len : 8,
    x_px : 48.,
    y_px : 156.,
};

impl AdderFPMain {
    pub fn draw_default_art(&self, adder : &Adder, name : &str) {
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

        // 创建主要区域的黑色矩形边框，以及黑色的subarea以外区域
        
        document = document.add(
        Rectangle::new()
            .set("x", BORDER)
            .set("y", BORDER)
            .set("width", art.x_len as f32 * art.x_px)
            .set("height", art.y_len as f32 * art.y_px)
            .set("fill", "black")
            .set("stroke", "none")
        );
        for sub_area in self.sub_area_dict.values() {
            let y = sub_area.y as f32;
            let x_left = sub_area.x_min as f32;
            let x_right = sub_area.x_max as f32;
            let width = x_right - x_left;
            document = document.add(
        Rectangle::new()
                .set("x", BORDER + x_left * art.x_px)
                .set("y", BORDER + (art.y_len as f32 - y - 1.) * art.y_px)
                .set("width", width * art.x_px)
                .set("height", art.y_px)
                .set("fill", "white")
                .set("stroke", "none")
            );
        }
        document = document.add(
        Rectangle::new()
            .set("x", BORDER)
            .set("y", BORDER)
            .set("width", art.x_len as f32 * art.x_px)
            .set("height", art.y_len as f32 * art.y_px)
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1)
        );

        // 画纵向网格线
        for x in 1..art.x_len - 1 {
            document = document.add(
            Line::new()
                    .set("x1", x as f32 * art.x_px + BORDER)
                    .set("y1", BORDER)
                    .set("x2", x as f32 * art.x_px + BORDER)
                    .set("y2", BORDER + art.y_len as f32 * art.y_px)
                    .set("stroke", "black")
                    .set("stroke-dasharray", "8, 16")
            );
        }

        // draw_cell can move one by one
        for (cell_id, cell_pos) in &self.cell_pos_dict {
            let color = if let Some(adder_cell) = adder.cells.get(cell_id.0 as usize) {
                adder_cell.1.node.logic.color_hex_inner()
            } else {
                "white"
            };
            let cell_info = self.cell_static_dict.get(cell_id).unwrap();
            let x_middle = cell_pos.x as f32;
            let width = cell_info.width as f32;
            let sub_area_id = cell_pos.sub_area_id;
            let y = self.sub_area_dict.get(&sub_area_id).unwrap().y as f32;

            let x_given = BORDER + (x_middle - width / 2.) * art.x_px;
            let y_given = BORDER + (art.y_len as f32 - y - 1.) * art.y_px;
            let width_given = width * art.x_px;
            let height_given = art.y_px;

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

            let x_given = BORDER + (x_middle) * art.x_px;
            let y_given = BORDER + (art.y_len as f32 - y -0.5) * art.y_px;

            document = document.add(Text::new(&format!("{}", cell_info.name.as_str()))
                .set("x", x_given)
                .set("y", y_given-40.)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", 20.)
            );
            let wire_num = cell_info.wires.len();
            let wire_x_force_marker = if self.given_cell_x_wire_force_atom(*cell_id) == 0. {
                " "
            } else if self.given_cell_x_wire_force_atom(*cell_id) > 0. {
                ">"
            } else {
                "<"
            };
            let wire_down_force_marker = match self.given_cell_y_wire_force_atom(*cell_id).get(&YMove::Down) {
                Some(v) => if *v > 0. { "v" } else { " " },
                None => " ",
            };
            let wire_up_force_marker = match self.given_cell_y_wire_force_atom(*cell_id).get(&YMove::Up) {
                Some(v) => if *v > 0. { "^" } else { " " },
                None => " ",
            };
            document = document.add(Text::new(&format!("{wire_num}{wire_x_force_marker}{wire_down_force_marker}{wire_up_force_marker}"))
                .set("x", x_given)
                .set("y", y_given+5.)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", 50.)
            );
            let wire_energu = self.given_cell_wire_energy(*cell_id) as i32;
            
            document = document.add(Text::new(&format!("{wire_energu}"))
                .set("x", x_given)
                .set("y", y_given+50.)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", 30.)
            );

        }

        // draw_cell can't move one by one
        for (cell_id, (x, y)) in &self.cell_fixed_pos_dict {
            let x = *x;
            let y = *y as f32;
            let color = "red";
            let cell_info = self.cell_static_dict.get(cell_id).unwrap();
            let x_middle = x as f32;
            let width = cell_info.width as f32;

            let x_given = BORDER + (x_middle - width / 2.) * art.x_px;
            let y_given = BORDER + (art.y_len as f32 - y - 1.) * art.y_px;
            let width_given = width * art.x_px;
            let height_given = art.y_px;

            document = document.add(Rectangle::new()
                .set("x", x_given)
                .set("y", y_given)
                .set("width", width_given)
                .set("height", height_given)
                .set("fill", color)
                .set("stroke", "black")
                .set("stroke-width", 1)
                .set("opacity", 0.5)
            );

            let x_given = BORDER + (x_middle) * art.x_px;
            let y_given = BORDER + (art.y_len as f32 - y -0.5) * art.y_px;

            document = document.add(Text::new(&format!("{}", cell_info.name.as_str()))
                .set("x", x_given)
                .set("y", y_given-50.)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", 40.)
                .set("fill", "white")
            );
            let wire_num = cell_info.wires.len();
            document = document.add(Text::new(&format!("{wire_num}"))
                .set("x", x_given)
                .set("y", y_given+5.)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", 50.)
                .set("fill", "white")
            );
            let wire_energu = self.given_cell_wire_energy(*cell_id) as i32;
            
            document = document.add(Text::new(&format!("{wire_energu}"))
                .set("x", x_given)
                .set("y", y_given+50.)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", 30.)
                .set("fill", "white")
            );

        }

        let all_wire_energy = self.all_wire_energy();
        document = document.add(Text::new(&format!("all wire energy : {all_wire_energy}"))
            .set("x", BORDER)
            .set("y", 0.)
            .set("font-family", "Arial")
            .set("font-size", 100.)
            .set("fill", "black")
        );

        // show density
        for (sub_area_id, sub_area) in &self.sub_area_dict {
            let x = sub_area.x_min;
            let y = sub_area.y;
            let density = self.given_sub_area_density(*sub_area_id);
            let density_format = format!("{}%", (density*100.).round() as i32);

            let x_given = BORDER - 150. + x as f32 * art.x_px;
            let y_given = BORDER + (art.y_len as f32 - y as f32 - 0.5) * art.y_px;

            let color = if density > 0.95 {
                "red"
            } else {
                "green"
            };

            document = document.add(Text::new(&format!("{density_format}"))
                .set("x", x_given)
                .set("y", y_given )
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", 100.)
                .set("fill", color)
            );
        }

        svg::save(&format!("{name}.svg"), &document).unwrap();
    }
}

// fn auto_wrap_for_given_txt(
//     content : &str,
//     x : f32,
//     y : f32,
//     width : f32,
//     height : f32,
//     font_size : f32,
//     color : &str,
// ) -> Text {

// }

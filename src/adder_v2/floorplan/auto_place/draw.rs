use svg::{Document, Node, node::element::{Line, Rectangle, Text}};

use crate::adder_v2::{draw::adder_draw::ToBeDraw, floorplan::auto_place::{PlaceData, PlacePos}};

const BORDER : f32 = 400.;
const H : f32 = 156.;
const W : f32 = 48.*4.;
const DEFAULT_ALL_W : f32 = 40.;
const DEFAULT_ALL_W_USIZE : usize = 40;

impl PlaceData {
    pub fn draw(&self) {
        let mut document = Document::new().set("viewBox", (0, 0, 
                    BORDER + W * DEFAULT_ALL_W, 
                    BORDER + H * 34.,
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
            .set("x", BORDER/2.)
            .set("y", BORDER/2.)
            .set("width", W * DEFAULT_ALL_W)
            .set("height", H * 34.)
            .set("fill", "white")
            .set("stroke", "black")
            .set("stroke-width", 1)
        );

        fn pos_to_xy(pos : &PlacePos) -> (f32, f32) {
            (BORDER/2. + W * pos.col as f32, BORDER/2. + H * 34. - H * pos.row as f32)
        }

        let mut cell_to_be_draw : Vec<Box<dyn Node>> = vec![];
        let mut left_m0_direct_info_to_be_draw : Vec<Box<dyn Node>> = vec![];

        // 绘制每一个cell
        for (cell_id, place_info) in self.data.iter() {
            let cell = &self.cells.get(*cell_id as usize).unwrap().1;
            let color = cell.node.logic.color_hex_inner();

            let (x, y) = pos_to_xy(&place_info.pos);

            // 主cell格子
            cell_to_be_draw.push(Box::new(Rectangle::new()
                .set("x", x)
                .set("y", y - H)
                .set("width", W * place_info.width as f32)
                .set("height", H)
                .set("fill", color)
                .set("stroke", "black")
                .set("stroke-width", 1)
            ));
            let mut out_wire_string = cell.node.io.output_z.1.to_string();
            if let Some(o1) = &cell.node.io.output_o1 {
                out_wire_string = format!("{}; {out_wire_string}", o1.1.to_string());
            }
            // wire文字
            cell_to_be_draw.push(Box::new(Text::new(&format!("{out_wire_string}"))
                .set("x", x+W*0.25)
                .set("y", y - H * 0.6)
                .set("font-family", "Arial")
                .set("font-size", 80.)
                .set("fill", "black")
            ));
            // cell ID
            cell_to_be_draw.push(Box::new(Text::new(&format!("{cell_id}"))
                .set("x", x+W*place_info.width as f32)
                .set("y", y)
                .set("font-family", "Arial")
                .set("font-size", 60.)
                .set("text-anchor", "end") // 向右对齐
                .set("fill", "black")
            ));
            // m0左直连个数
            let left_m0_direct_num = place_info.left_direct_wire_list.len();
            if left_m0_direct_num > 0 {
                left_m0_direct_info_to_be_draw.push(Box::new(Rectangle::new()
                    .set("x", x-30.)
                    .set("y", y - H/2.-50.)
                    .set("width", 60.)
                    .set("height", 90.)
                    .set("fill", "white")
                    .set("stroke", "red")
                    .set("stroke-width", 1)
                ));
                left_m0_direct_info_to_be_draw.push(Box::new(Text::new(&format!("{left_m0_direct_num}"))
                    .set("x", x)
                    .set("y", y-H/2.)
                    .set("font-family", "Arial")
                    .set("font-size", 100.)
                    .set("text-anchor", "middle") // 水平居中
                    .set("dominant-baseline", "middle")   // 垂直居中
                    .set("fill", "black")
                ));
            }
            
        }

        // 绘制
        for node in cell_to_be_draw {
            document = document.add(node);
        }
        for node in left_m0_direct_info_to_be_draw {
            document = document.add(node);
        }

        // 虚线网格
        for i in 0..DEFAULT_ALL_W_USIZE {
            document = document.add(Line::new()
                    .set("x1", BORDER/2.+i as f32 * W)
                    .set("y1", BORDER/2.)
                    .set("x2", BORDER/2.+i as f32 * W)
                    .set("y2", H * 34.+BORDER/2.)
                    .set("stroke", "black")
                    .set("stroke-dasharray", "8, 16"));
        }

        // 标注坐标
        for i in 0..DEFAULT_ALL_W_USIZE {
            document = document.add(Text::new(&format!("{i}"))
                .set("x", BORDER/2.+i as f32 * W + W/2.)
                .set("y", BORDER/4.)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", 100)
            );
            document = document.add(Text::new(&format!("{i}"))
                .set("x", BORDER/2.+i as f32 * W + W/2.)
                .set("y", BORDER * 0.75 + H * 34.)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", 100)
            );
        }

        // 直连数量


        svg::save("place.svg", &document).unwrap();
    }
}
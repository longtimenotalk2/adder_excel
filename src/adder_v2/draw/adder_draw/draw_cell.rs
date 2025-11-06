use svg::{node::element::{Line, Rectangle, Text}, Node};

use crate::adder_v2::{cell::cell_info::CellInfo, draw::{adder_draw::{AdderDraw, BigRuler, ToBeDraw}, adder_frame::{AdderFrame, CellPos, Pos, WirePos}}, logic::Logic, wire::Wire, Id};

impl AdderDraw {
    pub fn draw_cell(&self, 
        logic : &Logic, 
        info : &CellInfo, 
        pos : &Pos, 
        cell_pos : &CellPos,
        inputs : &[((Id, Wire), (Pos, CellPos, WirePos))],
        outputs : &[(Id, Wire)],
        ruler : &BigRuler,
        frame : &AdderFrame,
    ) -> ToBeDraw {
        let mut front : Vec<Box<dyn Node>> = vec![];
        let mut back : Vec<Box<dyn Node>> = vec![];

        let (cell_x, cell_y) = ruler.get_cell_xy(pos, cell_pos);

        // cell box
        let mark_vddh = self.show_vddh && info.is_power_vddh();
        let border_color = if mark_vddh {
            "red"
        } else {
            "black"
        };
        front.push(Box::new(Rectangle::new()
            .set("x", cell_x - self.cell_width / 2.)
            .set("y", cell_y - self.cell_height / 2.)
            .set("width", self.cell_width - 2.)
            .set("height", self.cell_height)
            .set("fill", logic.color_hex_inner())
            .set("stroke", border_color)
            .set("stroke-width", 2)
            .set("opacity", 0.8)
        ));

        // cell name
        let name_list = logic.string_for_show();
        if name_list.len() == 2 {
            let name0 = &name_list[0];
            let name1 = &name_list[1];
            front.push(Box::new(Text::new(&format!("{name0}"))
                .set("x", cell_x)
                .set("y", cell_y - self.font_cell_name / 2.)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", self.font_cell_name)
            ));
            front.push(Box::new(Text::new(&format!("{name1}"))
                .set("x", cell_x)
                .set("y", cell_y + self.font_cell_name / 2.)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", self.font_cell_name)
            ));
        } else {
            let name = &name_list[0];
            front.push(Box::new(Text::new(&format!("{name}"))
                .set("x", cell_x)
                .set("y", cell_y)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", self.font_cell_name)
            ));
        }

        // out wire name
        for (wire_pos, (_, wire)) in outputs.iter().enumerate() {
            let wire_pos = &WirePos::new(wire_pos);
            let wire_name = wire.to_string();
            let (x, y) = ruler.get_wire_xy(pos, cell_pos, wire_pos);

            front.push(Box::new(Text::new(&format!("{wire_name}"))
                .set("x", x)
                .set("y", y - self.wire_height / 2.)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", self.font_wire_name)
                .set("fill", border_color)
            ));
        }

        // input wire line
        let end_point = (cell_x, cell_y - self.cell_height / 2.);
        for (_wire, (input_pos, input_cell_pos, input_wire_pos)) in inputs.iter() {
            let start_point = ruler.get_wire_xy(input_pos, input_cell_pos, input_wire_pos);


            
            let input_mark_vddh = if input_pos.layer > 0 {
                let source_cell_info = &frame.frame.get(input_pos).unwrap()[input_cell_pos.0].cell_body.info;
                self.show_vddh && source_cell_info.is_power_vddh()
            } else {
                false
            };
            let input_border_color = if input_mark_vddh {
                "red"
            } else {
                "black"
            };

            back.push(Box::new(Line::new()
                .set("x1", start_point.0)
                .set("y1", start_point.1)
                .set("x2", end_point.0)
                .set("y2", end_point.1)
                .set("stroke", input_border_color)
                .set("stroke-width", self.wire_line_width)
            ));
        }

        ToBeDraw::new(front, back)
    }
}
use svg::{node::element::{Rectangle, Text}, Document};

use crate::adder_v2::{adder::Adder, draw::{adder_draw::{AdderDraw, ToBeDraw}, adder_frame::{AdderFrame, CellPos, Pos, WirePos}}, excel::{excel_to_datalist::ExcelDataList, ExcelFrame}, Id};

impl AdderDraw {
    pub fn draw(&self, frame : &AdderFrame, save_path : &str) {

        let ruler = self.get_big_ruler(frame);

        let mut document = Document::new().set("viewBox", (0, 0, 
                    ruler.full_width + self.border_left + self.border_right, 
                    ruler.full_height + self.border_down + self.border_up,
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

        dbg!(ruler.full_width);
        dbg!(ruler.full_height);

        // 创建主要区域的黑色矩形
        document = document.add(
        Rectangle::new()
            .set("x", self.border_left)
            .set("y", self.border_up)
            .set("width", ruler.full_width)
            .set("height", ruler.full_height)
            .set("fill", "white")
            .set("stroke", "black")
            .set("stroke-width", 1)
        );

        // layer标号
        let mut y = self.border_up + self.cell_y_interval + self.cell_height +  self.cell_height / 2.;
        for layer in 1..=frame.layer_max {
            document = document.add(
            Text::new(&format!("{layer}"))
                .set("x", self.border_left / 2.0)
                .set("y", y)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", self.font_index)
            );
            document = document.add(
            Text::new(&format!("{layer}"))
                .set("x", self.border_left + ruler.full_width + self.border_right / 2.0)
                .set("y", y)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", self.font_index)
            );
            y += self.cell_y_interval + self.cell_height;
        }

        // index 标号
        for index in 0..frame.bits {
            let (x, _) = ruler.get_cell_xy(&Pos::new(index, 1), &CellPos::default());
            document = document.add(
            Text::new(&format!("{index}"))
                .set("x", x)
                .set("y", self.border_up / 2.0)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", self.font_index)
            );
            document = document.add(
            Text::new(&format!("{index}"))
                .set("x", x)
                .set("y", self.border_up + ruler.full_height + self.border_down / 2.0)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", self.font_index)
            );
        }

        // 输入的ab

        for index in 0..frame.bits {
            let (x, y) = ruler.get_wire_xy(&Pos::new(index, 0), &CellPos::default(), &WirePos::default());
            document = document.add(
            Text::new(&format!("a{index}"))
                .set("x", x)
                .set("y", y - self.wire_height / 2.)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", self.font_wire_name)
            );
            let (x, y) = ruler.get_wire_xy(&Pos::new(index, 0), &CellPos::default(), &WirePos::new(1));
            document = document.add(
            Text::new(&format!("a{index}"))
                .set("x", x)
                .set("y", y - self.wire_height / 2.)
                .set("text-anchor", "middle") // 水平居中
                .set("dominant-baseline", "middle")   // 垂直居中
                .set("font-family", "Arial")
                .set("font-size", self.font_wire_name)
            );
        }


        let mut to_be_draw = ToBeDraw::default();

        for (pos, cells) in frame.frame.iter() {
            for (cell_pos, cell) in cells.iter().enumerate() {
                let cell_pos = CellPos::new(cell_pos);
                to_be_draw.update(self.draw_cell(
                    &cell.cell_body.logic, 
                    &cell.cell_body.info, 
                    pos, 
                    &cell_pos,
                    &cell.inputs,
                    &cell.outputs,
                    &ruler,
                    &frame,
                ));
            }
        }

        for back in to_be_draw.back.iter() {
            document = document.add(back.clone());
        }
        for front in to_be_draw.front.iter() {
            document = document.add(front.clone());
        }

        svg::save(save_path, &document).unwrap();
    }
}


#[test]
fn test_frame() {
    const PATH : &'static str = "src/adder_v2/project/a01_same_vt_vddh/excel/b08_t03_pn.txt";

    fn adder()  -> Adder {
        adder_and_excel().0
    }

    fn adder_and_excel()  -> (Adder, ExcelDataList<Id>) {
        let excel_frame = ExcelFrame::load(PATH);
        let excel_data_list = ExcelDataList::from_excel_frame(&excel_frame);
        let (adder, excel_map) = Adder::create_from_excel_data_list(excel_data_list, false, true);
        adder.check_id_all_match();
        (adder, excel_map)
    }
    
    let frame = AdderFrame::from_adder(&adder());
    let draw = AdderDraw::new();
    draw.draw(&frame, "adder.svg");
}
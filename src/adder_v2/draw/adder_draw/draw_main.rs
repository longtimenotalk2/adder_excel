use svg::{node::element::Rectangle, Document};

use crate::adder_v2::{adder::Adder, draw::{adder_draw::{AdderDraw, ToBeDraw}, adder_frame::{AdderFrame, CellPos}}, excel::{excel_to_datalist::ExcelDataList, ExcelFrame}, Id};

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
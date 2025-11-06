pub mod draw_logic;

use std::collections::BTreeMap;
use svg::{node::element::{Circle, Rectangle, Text}, Document};

use crate::adder_v2::{adder::Adder, draw::adder_frame::{AdderFrame, CellPos, Pos, WirePos}, excel::{excel_to_datalist::ExcelDataList, ExcelFrame}, Id};

pub struct AdderDraw {
    pub cell_width : f32,
    pub cell_height : f32,
    pub cell_x_interval : f32,
    pub cell_y_interval : f32,
    pub wire_x_interval : f32,
    pub wire_height : f32,
    pub border_up : f32,
    pub border_down : f32,
    pub border_left : f32,
    pub border_right : f32,
}

impl AdderDraw {
    pub fn new() -> Self {
        Self {
            cell_width: 50.,
            cell_height: 30.,
            cell_x_interval: 20.0,
            cell_y_interval: 20.0,
            wire_x_interval: 20.0,
            wire_height: 5.0,
            border_up: 20.0,
            border_down: 10.0,
            border_left: 20.0,
            border_right: 10.0,
        }
    }
}

pub struct BigRuler {
    full_height : f32,
    full_width : f32,
    cell_data : BTreeMap<Pos, BTreeMap<CellPos, (f32, f32)>>, // 记录某个整数坐标下，根据Cell Pos返回cell的绝对坐标
    wire_data : BTreeMap<Pos, BTreeMap<CellPos, BTreeMap<WirePos, (f32, f32)>>>, // 记录某个整数坐标下，根据Cell Pos以及Wire Pose，返回wire的绝对坐标
}

impl BigRuler {
    pub fn get_cell_xy(&self, pos : &Pos, cell_pos : &CellPos) -> (f32, f32) {
        self.cell_data.get(pos).unwrap().get(cell_pos).unwrap().clone()
    }

    pub fn get_wire_xy(&self, pos : &Pos, cell_pos : &CellPos, wire_pos : &WirePos) -> (f32, f32) {
        self.wire_data.get(pos).unwrap().get(cell_pos).unwrap().get(wire_pos).unwrap().clone()
    }
}

impl AdderDraw {
    pub fn get_big_ruler(&self, frame : &AdderFrame) -> BigRuler {
        let bits = frame.bits;
        
        let mut max_cell_len_with_index = vec![1; frame.bits];
        let mut layer_max = 0;
        for (pos, cells) in frame.frame.iter() {
            let index = pos.index;
            max_cell_len_with_index[index] = cells.len().max(max_cell_len_with_index[index]);
            layer_max = layer_max.max(pos.layer);
        }

        let mut full_width = (bits + 1) as f32 * self.cell_x_interval;
        for cell_len in max_cell_len_with_index.iter() {
            full_width += *cell_len as f32 * self.cell_width;
        }

        let full_height = (layer_max + 1) as f32 * self.cell_y_interval + layer_max as f32 * self.cell_height;

        let mut cell_data : BTreeMap<Pos, BTreeMap<CellPos, (f32, f32)>> = BTreeMap::new();
        let mut wire_data : BTreeMap<Pos, BTreeMap<CellPos, BTreeMap<WirePos, (f32, f32)>>> = BTreeMap::new();

        for (pos, cells) in frame.frame.iter() {
            let index = pos.index;
            let layer = pos.layer;
            let y = (self.cell_y_interval + self.cell_height) * (layer + 1) as f32 -  self.cell_height / 2.0 + self.border_up;
            let y_wire = y + self.cell_height / 2.0 + self.wire_height / 2.0;
            let mut x = full_width - ( 
                max_cell_len_with_index[0..index].iter().sum::<usize>() as f32 * self.cell_width + (index + 1) as f32 * self.cell_x_interval + self.cell_width / 2.0
            ) + self.border_left;
            for (cell_pos, cell) in cells.iter().enumerate() {
                x += self.cell_width;
                let cell_pos = CellPos::new(cell_pos);
                cell_data.entry(pos.clone()).or_default().insert(cell_pos.clone(), (x, y));
                let wire_len = cell.outputs.len();
                match wire_len {
                    1 => {
                        let mut inner = BTreeMap::new();
                        inner.insert(WirePos::default(), (x, y_wire));
                        wire_data.entry(pos.clone()).or_default().insert(cell_pos.clone(), inner);
                    },
                    2 => {
                        let mut inner = BTreeMap::new();
                        inner.insert(WirePos::default(), (x - self.wire_x_interval / 2.0 , y_wire));
                        inner.insert(WirePos::new(1), (x + self.wire_x_interval / 2.0, y_wire));
                        wire_data.entry(pos.clone()).or_default().insert(cell_pos.clone(), inner);
                    },
                    _ => unimplemented!(),
                }
            }
        }

        BigRuler {
            full_height,
            full_width,
            cell_data,
            wire_data,
        }
    }

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


        // for i in 0..frame.bits {
        //     let rec = Rectangle::new()
        //         .set("x", get_x(i, 0, 0))
        //         .set("y", 100)
        //         .set("width", self.cell_width)
        //         .set("height", self.cell_height)
        //         .set("text-anchor", "middle") // 水平居中
        //         .set("dominant-baseline", "middle")   // 垂直居中
        //         .set("fill", "blue")
        //         .set("stroke", "black")
        //         .set("stroke-width", 2);
        //     document = document.add(rec);
        // }

        let circle = Circle::new()
            .set("cx", 0) // 圆心的x坐标
            .set("cy", 0)  // 圆心的y坐标
            .set("r", 2)   // 半径
            .set("fill", "red"); // 填充颜色
        document = document.add(circle);

        let text = Text::new("AOAI211")
            .set("x", 50)
            .set("y", 50)
            .set("text-anchor", "middle") // 水平居中
            .set("dominant-baseline", "middle")   // 垂直居中
            .set("font-family", "Arial")
            .set("font-size", 10);
        document = document.add(text);

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
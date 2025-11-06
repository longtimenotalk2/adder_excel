use svg::{node::element::Rectangle, Node};

use crate::adder_v2::{cell::cell_info::CellInfo, draw::{adder_draw::{AdderDraw, BigRuler, ToBeDraw}, adder_frame::{CellPos, Pos, WirePos}}, logic::Logic, wire::Wire, Id};

impl AdderDraw {
    pub fn draw_cell(&self, 
        logic : &Logic, 
        info : &CellInfo, 
        pos : &Pos, 
        cell_pos : &CellPos,
        inputs : &[((Id, Wire), (Pos, CellPos, WirePos))],
        outputs : &[(Id, Wire)],
        ruler : &BigRuler,
    ) -> ToBeDraw {
        let mut front : Vec<Box<dyn Node>> = vec![];
        let mut back : Vec<Box<dyn Node>> = vec![];

        let (cell_x, cell_y) = ruler.get_cell_xy(pos, cell_pos);

        front.push(Box::new(Rectangle::new()
            .set("x", cell_x - self.cell_width / 2.)
            .set("y", cell_y - self.cell_height / 2.)
            .set("width", self.cell_width)
            .set("height", self.cell_height)
            .set("fill", logic.color_hex_inner())
            .set("stroke", "black")
            .set("stroke-width", 2)
        ));

        ToBeDraw::new(front, back)
    }
}
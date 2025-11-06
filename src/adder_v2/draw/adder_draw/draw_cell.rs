use svg::Node;

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



        ToBeDraw::new(front, back)
    }
}
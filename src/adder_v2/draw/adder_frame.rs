use std::collections::BTreeMap;

use crate::adder_v2::{adder::Adder, cell::cell_body::CellBody, excel::{excel_to_datalist::ExcelDataList, ExcelFrame}, logic::Logic, wire::{Flag, Wire}, Id};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos {
    pub index : usize,
    pub layer : i32,
}
#[derive(Debug, Clone, Default)]
pub struct CellPos(pub usize);

impl CellPos {
    pub fn new(index : usize) -> Self {
        Self(index)
    }
}

#[derive(Debug, Clone, Default)]
pub struct WirePos(pub usize);

impl WirePos {
    pub fn new(index : usize) -> Self {
        Self(index)
    }
}

#[derive(Debug, Clone)]
pub struct CellFrame {
    pub cell_body : CellBody,
    pub inputs : Vec<((Id, Wire), (Pos, CellPos, WirePos))>,
    pub outputs : Vec<(Id, Wire)>,
}

#[derive(Debug, Clone)]
pub struct AdderFrame {
    pub frame : BTreeMap<Pos, Vec<CellFrame>>,
    pub bits : usize,
}

impl AdderFrame {
    pub fn from_adder(adder : &Adder) -> Self {
        // 先整理所有cell的pos+CellPos，以及所有wire的pos+CellPos+WirePos
        let cell_layers = adder.scan_layer_absolute();
        
        let mut frame: BTreeMap<Pos, Vec<CellFrame>> = BTreeMap::new();

        let mut detected_wire: BTreeMap<(Id, Wire), (Pos, CellPos, WirePos)> = BTreeMap::new();

        for (i, (_, cell)) in adder.cells.iter().enumerate() {
            let layer = cell_layers[i];
            let index = cell.node.io.output_z.1.index;
            let pos = Pos { index, layer };
            let cell_pos = CellPos::new(frame.get(&pos).map(|l| l.len()).unwrap_or_default());

            let mut inputs = vec![];
            for (_port, wire) in cell.node.io.input.iter() {
                let wire_pos_full = if wire.1.is_input() {
                    (pos.clone(), CellPos::default(), if wire.1.flag == Flag::A {WirePos::default()} else {WirePos(1)})
                } else {
                    detected_wire.get(&wire).unwrap().clone()
                };
                inputs.push((wire.clone(), wire_pos_full.clone()));
            }

            let mut outputs = vec![];
            if let Some(wire) = &cell.node.io.output_o1 {
                let wire_pos = WirePos::new(outputs.len());
                outputs.push(wire.clone());
                detected_wire.insert(wire.clone(), (pos.clone(), cell_pos.clone(), wire_pos));
            }
            let wire = &cell.node.io.output_z;
            let wire_pos = WirePos::new(outputs.len());
            outputs.push(wire.clone());
            detected_wire.insert(wire.clone(), (pos.clone(), cell_pos, wire_pos));

            let cell_frame = CellFrame { cell_body: cell.to_cell_body(), inputs, outputs };
            frame.entry(pos).or_default().push(cell_frame);
        }

        Self {
            frame,
            bits : adder.bits,
        }
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
    dbg!(frame);
}
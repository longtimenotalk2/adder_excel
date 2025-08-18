use crate::std::{adder::{Adder, Cell, CellHinter}, node_create::LogicBlockMappingTable, wire::Wire};

impl Adder {
    pub fn create_by_cell_hint(
        bits : usize,
        input_is_neg : bool,
        output_is_neg : bool,
        hints : Vec<(Vec<Wire>, CellHinter, i32)>, // index
    ) -> Self {
        let mut cells = vec![];
        let mut history_wires = vec![];
        for (mut wires, hint, layer) in hints {
            let drive = hint.drive;

            let mut error_infos = String::new();
            let mut result = None;
            for logic_block_hint in hint.logic_block_hints {
                match LogicBlockMappingTable::create_from_wire_by_hint(
                    &wires.get(0).cloned().unwrap(),
                    logic_block_hint,
                    &history_wires,
                ) {
                    Ok(r) => {
                        result = Some(r);
                        break;
                    },
                    Err(e) => {
                        error_infos += &format!("{:?}\n", e);
                    }
                }
            }
            if let Some(logic_block_map) = result {
                cells.push(Cell {
                    logic_block_map,
                    drive,
                    custom_demand : vec![],
                    layer,
                    index : wires.get(0).unwrap().index,
                });
            } else {
                panic!("{}", error_infos);
            }
            history_wires.append(&mut wires);
        }
        
        Self {
            bits,
            input_is_neg,
            output_is_neg,
            cells,
        }
    }
}
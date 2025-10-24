use crate::adder_v2::{adder::{Adder, Cell, CellInfo}, excel::excel_to_datalist::ExcelDataList, node::{Node, NodeHint}, wire::{wire_list::WireList, Wire}, Id};

impl Adder {
    pub fn create_from_excel_data_list(
        excel_data_list: ExcelDataList<(NodeHint, CellInfo, Option<Vec<i32>>)>,
        input_is_neg: bool,
        output_is_neg: bool,
    ) -> (Self, ExcelDataList<Id>) {
        let bits = excel_data_list.bits;
        // 首先假设输入是正，生成全进位链
        let mut wire_id : Id = 0;

        let mut wires: Vec<(Id, Wire)> = vec![];
        let mut cells: Vec<(Id, Cell)> = vec![];
        let mut excel_cell_map: ExcelDataList<Id> = ExcelDataList::new_with_bits(bits);

        for i in 0..bits {
            wires.push((wire_id, Wire::from_str(&format!("a{i}"))));
            wire_id += 1;
            wires.push((wire_id, Wire::from_str(&format!("b{i}"))));
            wire_id += 1;
        }

        let mut wire_list = WireList(wires);

        for (cell_id, (excel_key, (hint, cell_info, _))) in excel_data_list.data.iter().enumerate() {
            let cell_id = cell_id as Id;
            match Node::create_from_hint(hint, &mut wire_list)  {
                Ok(node) => {
                    for output_wire in node.get_ordered_output_wires() {
                        wire_list.0.push(output_wire);
                    }
                    let cell = Cell::new(node, cell_info.clone());
                    cells.push((cell_id, cell));
                    excel_cell_map.data.insert(excel_key.clone(), cell_id);
                }
                Err(e) => {
                    println!("wire list : ");
                    for wire in &wire_list.0 {
                        println!("> {:02} : {}", wire.0, wire.1.to_string());
                    }
                    println!("{}", e.to_string());
                    panic!();
                }
            }
        }

        (
            Adder {
                bits,
                wires : wire_list.0,
                cells,
            },
            excel_cell_map,
        )
    }
}
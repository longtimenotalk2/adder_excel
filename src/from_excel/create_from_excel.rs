use crate::{from_excel::ExcelData, std::{adder::{Adder, CellHinter}, wire::Wire}};

impl ExcelData {
    pub fn create(&self, bits : usize, input_is_neg : bool, output_is_neg : bool) -> Adder {
        let mut hints = vec![];
        for node in &self.nodes {
            let cell_hinter = CellHinter::new(&node.wire_txt, &node.code_txt, node.index);
            let wires = match node.wire_txt.as_str() {
                "q~" => vec![
                    Wire::from_str_index("np", node.index),
                    Wire::from_str_index("q", node.index),
                ],
                "nq~" => vec![
                    Wire::from_str_index("ng", node.index),
                    Wire::from_str_index("nq", node.index),
                ],
                _ => {
                    let index = if node.wire_txt.starts_with('<') {
                        node.index + 1
                    } else {
                        node.index
                    };
                    vec![        
                        Wire::from_str_index(&node.wire_txt.replace("~", "").replace("<", ""), index),
                    ]
                }
            };
            let layer = node.layer;
            hints.push((wires, cell_hinter, layer));
        }
        Adder::create_by_cell_hint(bits, input_is_neg, output_is_neg, hints)
    }
}

#[test]
fn test_create() {
    let excel_data = ExcelData::load("src/from_excel/data/uf31.txt");
    let adder = excel_data.create(31, true, false);
    dbg!(adder);
}
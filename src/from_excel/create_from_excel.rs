use crate::{from_excel::ExcelData, std::{adder::{Adder, CellHinter}, wire::Wire}};

impl ExcelData {
    pub fn to_cell_hinter_list(&self) -> Vec<CellHinter> {
        let mut hints = vec![];
        for node in &self.nodes {
            let cell_hinter = CellHinter::new(&node.wire_txt, &node.code_txt, node.index, node.layer);
            hints.push(cell_hinter);
        }
        hints
    }
    
    pub fn create(&self, bits : usize, input_is_neg : bool, output_is_neg : bool) -> Adder {
        let mut hints = self.to_cell_hinter_list();
        Adder::create_by_cell_hint(bits, input_is_neg, output_is_neg, hints)
    }
}

#[test]
fn test_create() {
    let excel_data = ExcelData::load("src/from_excel/data/uf31.txt");
    let adder = excel_data.create(31, false, true);
    dbg!(adder);
}
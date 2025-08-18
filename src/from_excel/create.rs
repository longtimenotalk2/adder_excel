use crate::{from_excel::ExcelData, std::{adder::CellHinter, wire::Wire}};

impl ExcelData {
    pub fn create(&self) {
        for node in &self.nodes {
            let cell_hinter = CellHinter::new(&node.wire_txt, &node.code_txt, node.index);
            let wire = Wire::from_str_index(&node.wire_txt, node.index);
            dbg!(cell_hinter);
        }
    }
}

#[test]
fn test_create() {
    let excel_data = ExcelData::load("src/from_excel/data/uf31.txt");
    excel_data.create();
}
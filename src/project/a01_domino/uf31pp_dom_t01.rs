use crate::from_excel::ExcelData;

const PATH : &'static str = "src/project/a01_domino/excel_data/uf31pp_dom_t01.txt";

#[test]
fn test_load_excel() {
    let excel_data = ExcelData::load(PATH);
    dbg!(excel_data);
}

// #[test]
// fn test_to_cell_hinter() {
//     let excel_data = ExcelData::load(PATH);
//     let cell_hinter = excel_data.
//     dbg!(ExcelData::load(PATH));
// }
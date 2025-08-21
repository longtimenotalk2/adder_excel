use crate::from_excel::ExcelData;

const PATH : &'static str = "src/project/a01_domino/excel_data/uf31pp_base.txt";

#[test]
fn test_load_excel() {
    let excel_data = ExcelData::load(PATH);
    dbg!(excel_data);
}

#[test]
fn test_to_cell_hinter() {
    let excel_data = ExcelData::load(PATH);
    let cell_hinter = excel_data.to_cell_hinter_list();
    dbg!(cell_hinter);
}

#[test]
fn test_adder() {
    let excel_data = ExcelData::load(PATH);
    let adder = excel_data.create(31, false, false);
    dbg!(adder);
}

#[test]
fn test_function() {
    let excel_data = ExcelData::load(PATH);
    let adder = excel_data.create(31, false, false);
    // dbg!(adder);
    adder.function_check_random(1000, 0);
}
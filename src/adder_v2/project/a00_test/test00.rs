use crate::adder_v2::{adder::Adder, excel::{excel_to_datalist::ExcelDataList, ExcelFrame}};

const PATH : &'static str = "src/adder_v2/project/a00_test/excel/test00.txt";

#[test]
fn test_load_excel() {
    let excel_data = ExcelFrame::load(PATH);
    dbg!(excel_data);
}

#[test]
fn test_excel_data_list() {
    let excel_frame = ExcelFrame::load(PATH);
    let excel_data_list = ExcelDataList::from_excel_frame(&excel_frame);
    dbg!(excel_data_list);
}

#[test]
fn test_adder() {
    let excel_frame = ExcelFrame::load(PATH);
    let excel_data_list = ExcelDataList::from_excel_frame(&excel_frame);
    let (adder, _) = Adder::create_from_excel_data_list(excel_data_list, false, false);
    dbg!(adder);
}
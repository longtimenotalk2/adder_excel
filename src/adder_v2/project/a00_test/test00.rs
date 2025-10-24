use crate::adder_v2::excel::ExcelFrame;

const PATH : &'static str = "src/adder_v2/project/a00_test/excel/test00.txt";

#[test]
fn test_load_excel() {
    let excel_data = ExcelFrame::load(PATH);
    dbg!(excel_data);
}
use crate::adder_v2::{adder::Adder, excel::{excel_to_datalist::ExcelDataList, ExcelFrame}};

const PATH : &'static str = "src/adder_v2/project/a00_test/excel/test00.txt";

fn adder()  -> Adder {
    let excel_frame = ExcelFrame::load(PATH);
    let excel_data_list = ExcelDataList::from_excel_frame(&excel_frame);
    let (adder, _) = Adder::create_from_excel_data_list(excel_data_list, false, true);
    adder.check_id_all_match();
    adder
}

#[test]
fn test_adder() {
    let adder = adder();
}

#[test]
fn test_show_adder() {
    let adder = adder();
    println!("{}", adder.to_string());
}

#[test]
fn test_adder_function() {
    let adder = adder();
    adder.check_function_random(100);
}

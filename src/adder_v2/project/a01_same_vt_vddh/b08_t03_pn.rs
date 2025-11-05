use crate::adder_v2::{adder::Adder, cell_parse::Process, excel::{excel_to_datalist::ExcelDataList, ExcelFrame}, wire::Wire, Id};

const PATH : &'static str = "src/adder_v2/project/a01_same_vt_vddh/excel/b08_t03_pn.txt";

fn adder()  -> Adder {
    adder_and_excel().0
}

fn adder_and_excel()  -> (Adder, ExcelDataList<Id>) {
    let excel_frame = ExcelFrame::load(PATH);
    let excel_data_list = ExcelDataList::from_excel_frame(&excel_frame);
    let (adder, excel_map) = Adder::create_from_excel_data_list(excel_data_list, false, true);
    adder.check_id_all_match();
    (adder, excel_map)
}

#[test]
fn test_excel_frame() {
    let excel_frame = ExcelFrame::load(PATH);
    // dbg!(excel_frame);
    let excel_data_list = ExcelDataList::from_excel_frame(&excel_frame);
    dbg!(excel_data_list);
}

#[test]
fn test_adder() {
    let adder = adder();
    println!("{}", adder.to_string());
}

#[test]
fn test_adder_function() {
    let adder = adder();
    adder.check_function_random(1000);
}

#[test]
fn test_adder_cap() {
    let excel_frame = ExcelFrame::load(PATH);
    let (adder, excel_map) = adder_and_excel();
    adder.get_all_cap_by_excel(&excel_frame, &excel_map);
}

#[test]
fn test_adder_property() {
    let adder = adder();
    let set = adder.cell_body_set();
    for cell in set {
        println!("{}", cell.to_string())
    }
}

#[test]
fn test_cdl() {
    let adder = adder();
    let txt = adder.to_cdl("VDH_UFADDER_PN_B08_T03", Process::N3E);
    use std::fs::File;
    use std::io::prelude::*;
    let content = "This is the content to write to the file.";
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(txt.as_bytes());
}
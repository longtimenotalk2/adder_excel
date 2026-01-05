use crate::adder_v2::{adder::Adder, cell_parse::Process, draw::{adder_draw::AdderDraw, adder_frame::AdderFrame}, excel::{excel_to_datalist::ExcelDataList, ExcelFrame}, wire::Wire, Id};

const PATH : &'static str = "src/adder_v2/project/a07_dual_out_toggle/excel/uf31_v01_c01.txt";

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
    // dbg!(excel_frame);1
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

// #[test]
// fn test_adder_cap_debug() {
//     let adder = adder();
//     let wire = &adder.cells.get(000).unwrap().1.node.io.output_z;
//     dbg!(&wire);
//     let cap = adder.get_cap_cmos_for_wire(&wire);
//     dbg!(cap);
// }


#[test]
fn test_adder_property() {
    let adder = adder();
    let cell_num = adder.cell_num();
    println!("cell_num = {}", cell_num);
    let mos_num = adder.mos_num();
    println!("mos_num = {}", mos_num);
}

#[test]
fn test_draw() {
    let frame = AdderFrame::from_adder(&adder());
    let draw = AdderDraw::new();
    draw.draw(&frame, "adder.svg");
}

fn replaced() -> Vec<(String, String)> {
    let replaced = vec![
        ("XNR2D1_DUAL_OUT_BM156H3P48CPDELVT_EXCELSPECIAL_1".to_string(), "XNR2D1_DUAL_OUT_BM156H3P48CPDELVT_ULVT_LOWTOGGLE_V01".to_string()),
        ("XOR2D1_DUAL_OUT_BM156H3P48CPDELVT_EXCELSPECIAL_1".to_string(), "XOR2D1_DUAL_OUT_BM156H3P48CPDELVT_ULVT_LOWTOGGLE_V01".to_string()),
    ];
    replaced
}

#[test]
fn test_cdl() {
    let adder = adder();
    let mut txt = adder.to_cdl("ADDER_UF31_PN_V01_C01", Process::N3E);



    crate::adder_v2::cdl::find_and_replace(&mut txt, &replaced());

    use std::fs::File;
    use std::io::prelude::*;
    let content = "This is the content to write to the file.";
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(txt.as_bytes());
}

#[test]
fn test_netlist() {
    let adder = adder();
    let decr_info = adder.get_decr_cell_new_name(Process::N3E);
    let mut txt = adder.to_netlist("adder_uf31_pn_v01_c01", Process::N3E, decr_info);

    crate::adder_v2::cdl::find_and_replace(&mut txt, &replaced());
    use std::fs::File;
    use std::io::prelude::*;
    let content = "This is the content to write to the file.";
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(txt.as_bytes());
}
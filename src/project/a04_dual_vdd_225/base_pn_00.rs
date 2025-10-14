use crate::{cell_parse::ProcessAndProject, from_excel::ExcelData, std::adder::Adder};

const PATH : &'static str = "src/project/a04_dual_vdd_225/excel_data/base_pn_00.txt";

pub fn adder() -> Adder{
    let excel_data = ExcelData::load(PATH);
    let adder = excel_data.create_with_end_xnr_not_new(31, false, true, vec![22], vec![25]);
    adder
}

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
    let adder = adder();
    dbg!(adder);
}

#[test]
fn test_function() {
    let adder = adder();
    // dbg!(adder);
    adder.function_check_random(1000, 0);
}

#[test]
fn test_property() {
    let adder = adder();
    dbg!(adder.cells.len());
}

#[test]
fn test_cdl() {
    let adder = adder();
    let content = adder.to_cdl_std(ProcessAndProject::N3E1374, "UFADDER_PN_1342_H200");
    // let content = adder.to_cdl_all_vdd_split(ProcessAndProject::N4C1340, "UFADDER_PP_31");

    use std::fs::File;
    use std::io::prelude::*;
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(content.as_bytes());
}

#[test]
fn test_save_connect_relation() {
    let excel_data = ExcelData::load(PATH);
    let adder = excel_data.create_with_end_xnr_not_new(31, false, true, vec![22], vec![25]);
    adder.save_connect_relation(ProcessAndProject::N3E1374);
}


#[test]
fn test_sp() {
    let excel_data = ExcelData::load(PATH);
    let adder = excel_data.create(31, false, true);
    let adder_name = "UFADDER_PN_1342_H200";
    // let adder_cdl_path = "cdl/UFADDER_PP_31.cdl";
    let adder_cdl_path = "cdl/UFADDER_PN_1342_H200.cdl";
    
    let content = adder.create_sp_of_adder_timing_base_1(ProcessAndProject::N4C1342H200, adder_name, adder_cdl_path);


    use std::fs::File;
    use std::io::prelude::*;
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(content.as_bytes());
}
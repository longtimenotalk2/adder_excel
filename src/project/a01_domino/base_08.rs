use crate::{cell_parse::ProcessAndProject, from_excel::ExcelData};

const PATH : &'static str = "src/project/a01_domino/excel_data/uf31pp_b08.txt";

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
    adder.function_check_random(100, 0);
}

#[test]
fn test_property() {
    let excel_data = ExcelData::load(PATH);
    let adder = excel_data.create(31, false, false);
    dbg!(adder.cells.len());
}

#[test]
fn test_cdl() {
    let excel_data = ExcelData::load(PATH);
    let adder = excel_data.create_with_end_xnr_not_new(31, false, false, vec![], vec![25]);
    let content = adder.to_cdl_std(ProcessAndProject::N4C1340, "UFADDER_PP_31_BASE_08");
    // let content = adder.to_cdl_all_vdd_split(ProcessAndProject::N4C1340, "UFADDER_PP_31");

    use std::fs::File;
    use std::io::prelude::*;
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(content.as_bytes());
}


#[test]
fn test_sp() {
    let excel_data = ExcelData::load(PATH);
    let adder = excel_data.create(31, false, false);
    let adder_name = "UFADDER_PP_31_BASE_04";
    // let adder_cdl_path = "cdl/UFADDER_PP_31.cdl";
    let adder_cdl_path = "cdl/UFADDER_PP_31_BASE_04.cdl";
    
    // let content = adder.create_sp_of_adder_function(ProcessAndProject::N4C1340, adder_name, adder_cdl_path);
    // let content = adder.create_sp_of_adder_power_all_vdd_split(ProcessAndProject::N4C1340, adder_name, adder_cdl_path);
    let content = adder.create_sp_of_adder_timing_single(ProcessAndProject::N4C1340, adder_name, adder_cdl_path, 8);
    // let content = adder.create_sp_of_adder_power_2nd(ProcessAndProject::N4C1340, adder_name, adder_cdl_path, 7);

    use std::fs::File;
    use std::io::prelude::*;
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(content.as_bytes());
}
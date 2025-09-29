use crate::{cell_parse::ProcessAndProject, from_excel::ExcelData, std::adder::Adder};

const PATH : &'static str = "src/project/a03_domino_e07/excel_data/b24.txt";
// 修前序
const INPUT_IS_NEG : bool = false;
const OUTPUT_IS_NEG : bool = false;

fn adder_create() -> Adder {
    let excel_data = ExcelData::load(PATH);
    excel_data.create_with_end_xnr_not_new(31, INPUT_IS_NEG, OUTPUT_IS_NEG, vec![], vec![])
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
    let adder = adder_create();
    dbg!(adder);
}

#[test]
fn test_function() {
    let adder = adder_create();
    adder.function_check_random(1000, 0);
}

#[test]
fn test_cap() {
    let excel_data = ExcelData::load(PATH);
    let adder = adder_create();
    let caps = adder.capi_calc();
    // dbg!(caps.len());
    excel_data.cap_check(&caps);
    excel_data.cap_print(&caps);
}

#[test]
fn test_property() {
    let adder = adder_create();
    dbg!(adder.cells.len());
}

#[test]
fn test_cdl() {
    let adder = adder_create();
    let content = adder.to_cdl_std(ProcessAndProject::N4C1340, "UFADDER_PP_31_B23");

    use std::fs::File;
    use std::io::prelude::*;
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(content.as_bytes());
}


#[test]
fn test_sp() {
    let adder = adder_create();
    let adder_name = "UFADDER_PP_31_BASE00";
    // let adder_cdl_path = "cdl/UFADDER_PP_31.cdl";
    let adder_cdl_path = "cdl/UFADDER_PP_31_BASE00.cdl";

    let content = adder.timing_all_pre_two_inv(
        ProcessAndProject::N4C1340, 
        adder_name, 
        adder_cdl_path,
        true,
        false,
        true,
    );
    
    // let content = adder.create_sp_of_adder_function(ProcessAndProject::N4C1340, adder_name, adder_cdl_path);
    // let content = adder.create_sp_of_adder_power_all_vdd_split(ProcessAndProject::N4C1340, adder_name, adder_cdl_path);
    // let content = adder.create_sp_of_adder_timing_single(ProcessAndProject::N4C1340, adder_name, adder_cdl_path, 8);
    // let content = adder.create_sp_of_adder_power_2nd(ProcessAndProject::N4C1340, adder_name, adder_cdl_path, 7);

    use std::fs::File;
    use std::io::prelude::*;
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(content.as_bytes());
}
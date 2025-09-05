use crate::{cell_parse::ProcessAndProject, from_excel::ExcelData};

const PATH : &'static str = "src/project/a01_domino/excel_data/uf31pp_dom_t02.txt";

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
    dbg!(&adder);
    adder.function_check_random(100, 0);
}

#[test]
fn test_property() {
    let excel_data = ExcelData::load(PATH);
    let adder = excel_data.create(31, false, false);
    // dbg!(adder.all_abstract_cells());
    dbg!(adder.cells.len());
    dbg!(adder.cells.iter().filter(|c| c.custom_demand.len()>0).count());
}

#[test]
fn test_cdl() {
    let excel_data = ExcelData::load(PATH);
    let adder = excel_data.create(31, false, false);
    let content = adder.to_cdl_std(ProcessAndProject::N4C1340, "DOMINO_UF_PP_31");

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
    let adder_name = "DOMINO_UF_PP_31";
    let adder_cdl_path = "cdl/DOMINO_UF_PP_31_T02.cdl";
    
    // let content = adder.create_sp_of_adder_timing_base_0_sp(ProcessAndProject::N4C1340, adder_name, adder_cdl_path, true, false);
    let content = adder.create_sp_of_adder_timing_base_1_sp(ProcessAndProject::N4C1340, adder_name, adder_cdl_path, true, false);
    // let content = adder.create_sp_of_adder_timing_base_1(ProcessAndProject::N4C1340, adder_name, adder_cdl_path);
    // let content = adder.create_sp_of_adder_function(ProcessAndProject::N4C1340, adder_name, adder_cdl_path);
    // let content = adder.create_sp_of_adder_power(ProcessAndProject::N4C1340, adder_name, adder_cdl_path);

    use std::fs::File;
    use std::io::prelude::*;
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(content.as_bytes());
}
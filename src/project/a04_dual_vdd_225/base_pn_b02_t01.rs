use std::collections::{BTreeMap, BTreeSet};

use crate::{cell_parse::{ProcessAndProject, RealCell}, from_excel::ExcelData, std::adder::{Adder, CustomDemand}};

const PATH : &'static str = "src/project/a04_dual_vdd_225/excel_data/b02_t01.txt";

pub fn adder() -> Adder{
    let excel_data = ExcelData::load(PATH);
    let adder = excel_data.create_with_end_xnr_not_new(31, false, true, vec![], vec![25]);
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
fn test_check_high_match() {
    let adder = adder();
    adder.high_match_check();
}

#[test]
fn test_cap() {
    let excel_data = ExcelData::load(PATH);
    let adder = adder();
    let caps = adder.capi_calc();
    // dbg!(caps.len());
    excel_data.cap_check(&caps);
    excel_data.cap_print(&caps);
}

#[test]
fn test_property() {
    let adder = adder();
    dbg!(adder.cells.len());
}

#[test]
fn test_cdl() {
    let adder = adder();
    let content = adder.to_cdl_std(ProcessAndProject::N3E1374, "DUAL_VDD_UFADDER_PN_B02_T01");
    // let content = adder.to_cdl_all_vdd_split(ProcessAndProject::N4C1340, "UFADDER_PP_31");

    use std::fs::File;
    use std::io::prelude::*;
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(content.as_bytes());
}

#[test]
fn test_netlist() {
    let adder = adder();
    let content = adder.to_netlist("adder_ultrafast_31_l7_pn_SPCO_8", ProcessAndProject::N3E1374);
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
    let adder = adder();
    adder.save_connect_relation(ProcessAndProject::N3E1374);
}

#[test]
fn test_see_h2l_cell() {
    let adder = adder();
    for (id, port_and_wires) in adder.high_to_low_map().iter() {
        let cell = adder.cells.get(*id).unwrap();
        let inst_name = cell.inst_name();
        let abstract_cell = &cell.to_abstract_cell();
        let real_cell = RealCell::parse(ProcessAndProject::N3E1374, abstract_cell);
        let mut new_cell_name = real_cell.name.clone();
        new_cell_name += "_H2L";
        for (port, _wire) in port_and_wires.iter() {
            new_cell_name += &format!("_{}", port.0);
        };
        // println!("{}: {:?}", inst_name, port_and_wires);
        println!("{}: {}", inst_name, new_cell_name);
    }
}

#[test]
fn test_cell_list_for_3_type() {
    let mut l2h_cells = BTreeSet::new();
    let mut h2h_cells = BTreeSet::new();
    let mut h2l_cells = BTreeSet::new();

    let mut cells = vec![];
    let adder = adder();
    // L2H & H2H
    for cell in &adder.cells {
        if let Some(custom_demand) = cell.custom_demand.get(0) {
            if let CustomDemand::DualVdd(dual_vdd) = custom_demand {
                if dual_vdd.is_l2h() {
                    l2h_cells.insert(RealCell::parse(ProcessAndProject::N3E1374, &cell.to_abstract_cell()).name);
                } else if  dual_vdd.is_h2h() {
                    h2h_cells.insert(RealCell::parse(ProcessAndProject::N3E1374, &cell.to_abstract_cell()).name);
                }
            }
        }
    }
    let mut h2l_origin = BTreeSet::new();
    let mut h2l_mapping = BTreeMap::new();
    // H2L
    for (id, port_and_wires) in adder.high_to_low_map().iter() {
        let cell = adder.cells.get(*id).unwrap();
        let abstract_cell = &cell.to_abstract_cell();
        let real_cell = RealCell::parse(ProcessAndProject::N3E1374, abstract_cell);
        let old_cell_name = real_cell.name;
        let mut new_cell_name = old_cell_name.clone();
        h2l_origin.insert(new_cell_name.clone());
        new_cell_name += "_H2L";
        for (port, _wire) in port_and_wires.iter() {
            new_cell_name += &format!("_{}", port.0);
        };
        h2l_cells.insert(new_cell_name.clone());
        h2l_mapping.insert(new_cell_name, old_cell_name);
    }

    for cell in &l2h_cells {
        cells.push(cell.clone());
    }
    for cell in &h2h_cells {
        cells.push(cell.clone());
    }
    for cell in &h2l_cells {
        cells.push(cell.clone());
    }

    /*
    L2H * 4
    H2H * 8
    H2L * 9
    all = 21
    */
    assert_eq!(cells, vec![
        "INVD1BM156H3P48CPDELVT_1_P_ULVT_L2H_V03".to_string(),
        "NR2D1BM156H3P48CPDELVT_1_P_ULVT_L2H_V02".to_string(),
        "OAI21D1BM156H3P48CPDELVT_P_ULVT_L2H_V02".to_string(),
        "OAI21D2BM156H3P48CPDELVT_P_ULVT_L2H_V02".to_string(),
        "AOI21D1BM156H3P48CPDELVT_H2H_V03".to_string(),
        "AOI21D2BM156H3P48CPDELVT_H2H_V03".to_string(),
        "IAOI21D1BM156H3P48CPDELVT_H2H_V03".to_string(),
        "INVD1BM156H3P48CPDELVT_1_H2H_V03".to_string(),
        "ND2D1BM156H3P48CPDELVT_1_H2H_V03".to_string(),
        "NR2D1BM156H3P48CPDELVT_1_H2H_V02".to_string(),
        "OAI21D1BM156H3P48CPDELVT_H2H_V02".to_string(),
        "OAI21D2BM156H3P48CPDELVT_H2H_V02".to_string(),
        "IAOI21D1BM156H3P48CPDELVT_H2L_A1_A2".to_string(),
        "INVD1BM156H3P48CPDELVT_1_H2L_I".to_string(),
        "INVD2BM156H3P48CPDELVT_1_H2L_I".to_string(),
        "NR2D1BM156H3P48CPDELVT_1_H2L_A2".to_string(),
        "OAI21D1BM156H3P48CPDELVT_H2L_A2".to_string(),
        "OAO211D1BM156H3P48CPDELVT_H2L_A1".to_string(),
        "OAOI211D1BM156H3P48CPDELVT_H2L_A1".to_string(),
        "OR2D1BM156H3P48CPDELVT_1_H2L_A2".to_string(),
        "XNR2D1BM156H3P48CPDELVT_H2L_A1".to_string(),
    ]);

    // cell list show
    for cell in cells {
        // println!("{}", cell);
        // println!("cp /ic/projects/BM1374/public/5_custom/release/stdcell/stdcell_BM/elvt/spf/Cbest45/{}.Cbest45.spf .", cell);
        println!("cp /ic/projects/BM1374/users/haiwei.li/V0/work/spf/out/{}/{}.Cbest45.spf .", cell, cell);
    }



    // for cell in h2l_origin {
    //     // println!("{}", cell);
    //     // println!("cp /ic/projects/BM1374/public/5_custom/release/stdcell/stdcell_BM/elvt/spf/Cbest45/{}.Cbest45.spf .", cell);
    //     println!("cp /ic/projects/BM1374/public/5_custom/release/custom/elvt/spf/Cbest45/{}.Cbest45.spf .", cell);
    // }

    // give gds cp 
    // for (cell_new, cell_old) in h2l_mapping {
    //     // println!("{cell_new} {cell_old}");
    //     // println!("cp /ic/projects/BM1374/users/haiwei.li/V0/cds/{}.gds .", cell);
    //     // println!("cp /ic/projects/BM1374/public/5_custom/release/stdcell/stdcell_BM/elvt/gds/{}.gds .", cell);
    // }
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
pub mod mb_function;
pub mod delay_model;
pub mod random;
pub(crate) mod multi_vdd_test;
pub mod measure_adder;
pub mod cell;

use crate::hspice::line_cell;

#[test]
fn test() {
    let mut s = String::new();
    let v0 = "V0";
    let v1 = "V1";
    let kn = "KN";
    let r = "R";
    let rn = "RN";
    let inst_carry = "CARRY_P_TEST_V03";

    let n = 4;

    s += &line_cell("CARRY_00", &[
        kn.to_string(), // AG
        v1.to_string(), // AP
        v0.to_string(), // APN
        v0.to_string(), // CI
        format!("CN_{:02}", 0), // CN
        format!("CO_{:02}", 0), // CO
        kn.to_string(), // KN
        r.to_string(), // R
        rn.to_string(), // RN
        "VBB".to_string(),
        "VDD".to_string(),
        "VPP".to_string(),
        "VSS".to_string(),
    ], inst_carry);



    for i in 1..n {
        s += &line_cell(&format!("CARRY_{i:02}"), &[
            v1.to_string(), // AG
            v0.to_string(), // AP
            v1.to_string(), // APN
            format!("CO_{:02}", i-1), // CI
            // if i % 4 == 0 {format!("CO_{:02}", 0)} else {format!("CO_{:02}", i-1)}, // CI
            format!("CN_{:02}", i), // CN
            format!("CO_{:02}", i), // CO
            kn.to_string(), // KN
            r.to_string(), // R
            rn.to_string(), // RN
            "VBB".to_string(),
            "VDD".to_string(),
            "VPP".to_string(),
            "VSS".to_string(),
        ], inst_carry);
    }

    s += "\n";

    let inst_xnr = "XNR2SAMZD1BM200H6P51CNODELVT";

    for i in 0..n {
        s += &line_cell(&format!("XNR_{i:02}"), &[
            format!("CN_{:02}", i), // A1
            v0.to_string(), // A2
            "VBB".to_string(),
            "VDD".to_string(),
            "VPP".to_string(),
            "VSS".to_string(),
            format!("ZN_{:02}", i), // ZN
        ], inst_xnr);
    }

    use std::fs::File;
    use std::io::prelude::*;
    let content = "This is the content to write to the file.";
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(s.as_bytes());
}
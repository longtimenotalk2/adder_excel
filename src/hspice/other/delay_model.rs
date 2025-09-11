use crate::hspice::*;

const ND2: &'static str = "ND2MZD1BWP200H6P51CNODELVT";
const NR2: &'static str = "NR2MZD1BWP200H6P51CNODELVT";
const ND3: &'static str = "ND3MZD1BWP200H6P51CNODELVT";
const NR3: &'static str = "NR3MZD1BWP200H6P51CNODELVT";

#[test]
fn test_delay_model_base() {
    let n = 1;
    let dn = 1;
    let mut s = String::new();

    s += &line_subckt("BLOCK_EVEN", &["I", "VBB", "VDD", "VPP", "VSS", "ZN"]);
    s += &line_source_dc("HIGH", "avdd");
    for i in 0..n {
        let out = if i >= dn { &format!("ZN_FLOAT_{}", i) } else { "ZN" };
        s += &line_cell(&format!("CELL_EVEN_{i}"), &["I", "HIGH", "VBB", "VDD", "VPP", "VSS", out], ND2);
    }
    s += &line_end_subckt();
    s += &line_subckt("BLOCK_ODD", &["I", "VBB", "VDD", "VPP", "VSS", "ZN"]);
    s += &line_source_dc("LOW", "0");
    for i in 0..n {
        let out = if i >= dn { &format!("ZN_FLOAT_{}", i) } else { "ZN" };
        s += &line_cell(&format!("CELL_ODD_{i}"), &["I", "LOW", "VBB", "VDD", "VPP", "VSS", out], NR2);
    }
    s += &line_end_subckt();
    s += "\n";

    s += &line_source_period("I", "0", "avdd", 0., 1., Some(2.));
    s += "\n";

    for i in 0..10 {
        let input = if i == 0 { "I" } else { &format!("ZN_{}", i-1) };
        let output = &format!("ZN_{}", i);
        let block = if i % 2 == 0 { "BLOCK_EVEN" } else { "BLOCK_ODD" };
        let name = &format!("BLOCK_{i}");
        s += &line_cell(name, &[input, "VBB", "VDD", "VPP", "VSS", output], block);
    }

    s += "\n";

    s += &line_measure_delay("delay_stack_1", "ZN_7", "ZN_3", false, 1, false, 1);
    s += &line_measure_delay("delay_stack_2", "ZN_7", "ZN_3", true, 1, true, 1);


    use std::fs::File;
    use std::io::prelude::*;
    let content = "This is the content to write to the file.";
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(s.as_bytes());
}

#[test]
fn test_delay_model_3stack() {
    let n = 6;
    let dn = 1;
    let mut s = String::new();

    s += &line_subckt("BLOCK_EVEN", &["I", "VBB", "VDD", "VPP", "VSS", "ZN"]);
    s += &line_source_dc("HIGH", "avdd");
    for i in 0..n {
        let out = if i >= dn { &format!("ZN_FLOAT_{}", i) } else { "ZN" };
        s += &line_cell(&format!("CELL_EVEN_{i}"), &["I", "HIGH", "HIGH", "VBB", "VDD", "VPP", "VSS", out], ND3);
    }
    s += &line_end_subckt();
    s += &line_subckt("BLOCK_ODD", &["I", "VBB", "VDD", "VPP", "VSS", "ZN"]);
    s += &line_source_dc("LOW", "0");
    for i in 0..n {
        let out = if i >= dn { &format!("ZN_FLOAT_{}", i) } else { "ZN" };
        s += &line_cell(&format!("CELL_ODD_{i}"), &["I", "LOW", "LOW", "VBB", "VDD", "VPP", "VSS", out], NR3);
    }
    s += &line_end_subckt();
    s += "\n";

    s += &line_source_period("I", "0", "avdd", 0., 1., Some(2.));
    s += "\n";

    for i in 0..10 {
        let input = if i == 0 { "I" } else { &format!("ZN_{}", i-1) };
        let output = &format!("ZN_{}", i);
        let block = if i % 2 == 0 { "BLOCK_EVEN" } else { "BLOCK_ODD" };
        let name = &format!("BLOCK_{i}");
        s += &line_cell(name, &[input, "VBB", "VDD", "VPP", "VSS", output], block);
    }

    s += "\n";

    s += &line_measure_delay("delay_stack_1", "ZN_7", "ZN_3", false, 1, false, 1);
    s += &line_measure_delay("delay_stack_3", "ZN_7", "ZN_3", true, 1, true, 1);


    use std::fs::File;
    use std::io::prelude::*;
    let content = "This is the content to write to the file.";
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(s.as_bytes());
}
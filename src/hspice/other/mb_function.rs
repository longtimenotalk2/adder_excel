use crate::hspice::*;

#[test]
fn test_mb_function() {
    let mut txt = String::new();
    // 32bit MB
    for i in 0..32 {
        txt += &line_source_period(&format!("D{i:02}"), "0", "avdd", 0., 3., Some(6.));
    }
    txt += &line_source_period(&format!("CP"), "0", "avdd", 1., 1., Some(3.));

    txt += "\n";

    txt += &line_inc("cell/MB32DYNLHQNSAMZD1R34BM200H6P51CNODLVT_ELVT.Cbest60.spf");

    txt += "\n";

    let mut pins = vec!["CP".to_string()];
    for i in 0..32 {
        pins.push(format!("S{i:02}"));
    }
    for i in 0..32 {
        pins.push(format!("NQ{i:02}"));
    }

    pins.append(&mut vec!["VBB".to_string(), "VDD".to_string(), "VPP".to_string(), "VSS".to_string()]);

    txt += &line_cell("my_mb", &pins, "MB32DYNLHQNSAMZD1R34BM200H6P51CNODLVT_ELVT");

    use std::fs::File;
    use std::io::prelude::*;
    let content = "This is the content to write to the file.";
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(txt.as_bytes());
}
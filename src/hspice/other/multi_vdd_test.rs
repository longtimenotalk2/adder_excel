use crate::hspice::{test_out::measure_delay, *};

#[test]
fn test_from_210_to_270_vddh() {
    let mut txt = String::new();;

    let vdd_0 = 210;

    for i in 0..=12 { // 12
        let vdd = vdd_0 + i * 5;
        let vdd_label = format!("avdd_{vdd}");
        let vdd_pg_label = format!("VDDH_{vdd}");
        let zn_label = format!("ZN_{vdd}");
        txt += &format!(".PARAM {vdd_label}={vdd}m\n");
        txt += &line_source_dc(&vdd_pg_label, &vdd_label);
        txt += &line_cell(&format!("inv_{vdd}"), &[
            "I",
            "VBB",
            vdd_pg_label.as_str(),
            "VPP",
            "VSS",
            zn_label.as_str(),
        ], "INVD1BM156H3P48CPDELVT_1");
        txt += &line_cap(zn_label.as_str(), "c_fin");
        txt += &line_measure_power(&vdd_pg_label);
        // txt += &line_measure_delay_with_given_pg(
        //     &format!("vddh_{vdd}_f_50p"), 
        //     zn_label.as_str(), 
        //     "I", 
        //     true,
        //     1,
        //     "avdd/2",
        //     false,
        //     1,
        //     format!("{vdd_label}/2").as_str(),
        // );
        // txt += &line_measure_delay_with_given_pg(
        //     &format!("vddh_{vdd}_r_50p"), 
        //     zn_label.as_str(), 
        //     "I", 
        //     false,
        //     1,
        //     "avdd/2",
        //     true,
        //     1,
        //     format!("{vdd_label}/2").as_str(),
        // );
        // txt += &format!(".probe v({zn_label})\n");
        txt += "\n";
    }

    use std::fs::File;
    use std::io::prelude::*;
    let content = "This is the content to write to the file.";
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(txt.as_bytes());
}

#[test]
fn test_from_210_0_to_m60_vssl() {
    let mut txt = String::new();

    let vss_0 = 0;

    for i in 0..=12 { // 12
        let vss = vss_0 + i * 5;
        let vss_label = format!("avssl_{vss:03}");
        let vss_pg_label = format!("VSSL_{vss:03}");
        let zn_label = format!("ZN_{vss:03}");
        txt += &format!(".PARAM {vss_label}=-{vss}m\n");
        txt += &line_source_dc(&vss_pg_label, &vss_label);
        txt += &line_cell(&format!("inv_{vss}"), &[
            "I",
            "VBB",
            "VDD", 
            "VPP",
            vss_pg_label.as_str(),
            zn_label.as_str(),
        ], "INVD1BM156H3P48CPDELVT_1");
        txt += &line_cap(zn_label.as_str(), "c_fin");
        // txt += &line_measure_power(&vss_pg_label);
        // txt += &line_measure_delay_with_given_pg(
        //     &format!("vssl_{vss}_f_50p"), 
        //     zn_label.as_str(), 
        //     "I", 
        //     true,
        //     1,
        //     "avdd/2",
        //     false,
        //     1,
        //     format!("{}m", ((210+vss) as f64/2.)-vss as f64).as_str(),
        // );
        // txt += &line_measure_delay_with_given_pg(
        //     &format!("vssl_{vss}_r_50p"), 
        //     zn_label.as_str(), 
        //     "I", 
        //     false,
        //     1,
        //     "avdd/2",
        //     true,
        //     1,
        //     format!("{}m", ((210+vss) as f64/2.)-vss as f64).as_str(),
        // );
        txt += &format!(".probe v({zn_label})\n");
        txt += "\n";
    }

    use std::fs::File;
    use std::io::prelude::*;
    let content = "This is the content to write to the file.";
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(txt.as_bytes());
}
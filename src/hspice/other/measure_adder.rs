use crate::{adder_v2::cdl, hspice::line_measure_delay_with_td};

use super::*;

#[test]
fn test_measure() {
    /*
    对于输入是p的情况，
    base0时，b全1，a全0，然后有一个a先上升再下降 
    base1时，b是1后面跟全0，a全1，然后有一个a先下降再上升
    因此，pn adder，source的measure是
    base0_r是rise to rise，偶数的td
    base0_f是fall to fall，奇数的td
    base1_r是rise to rise，奇数的td
    base1_f是fall to fall，偶数的td

    pp adder的measure是
    base0_r是rise to fall，偶数的td
    base0_f是fall to rise，奇数的td
    base1_r是rise to fall，奇数的td
    base1_f是fall to rise，偶数的td
    
    */
    let bits = 31;
    let base = 1;
    let source_is_rise = false;
    let target_is_rise = true;
    let is_odd = 0; // 1奇数，0偶数

    let rise_txt = if source_is_rise {"r"} else {"f"};
    let mut txt = String::new();
    for i in 0..bits {
        let source_wire = format!("A{i:02}");
        for j in i..bits {
            let target_wire = format!("S{j:02}");
            
            txt += &line_measure_delay_with_td(
                &format!("base_{base}_{source_wire}_to_{target_wire}_{rise_txt}"),
                &target_wire, 
                &source_wire, 
                source_is_rise, 
                Some(format!("td+clkper*{}", 2*i+is_odd)),
                1, 
                target_is_rise, 
                Some(format!("td+clkper*{}", 2*i+is_odd)),
                1
            );
        }
        
    }
    txt += "\n";

    use std::fs::File;
    use std::io::prelude::*;
    let content = "This is the content to write to the file.";
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(txt.as_bytes());
}
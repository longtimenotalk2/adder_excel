use super::super::*;

#[test]
fn test_measure_delay() {
    let mut s = String::new();
    for i in 7..15 {
        // s += &line_measure_delay_with_td(
        //     &format!("h0_to_h1_at_a{:02}_r", i), 
        //     "x_adder.nh15_8", 
        //     "x_adder.nh7_0", 
        //     false, 
        //     Some(format!("td+clkper*{}", i*2+1)), 
        //     1, 
        //     true, 
        //     Some(format!("td+clkper*{}", i*2+1)), 
        //     1,
        // );
        s += &line_measure_delay_with_td(
            &format!("p1_to_h1_at_a{:02}_r", i), 
            "x_adder.nh15_8", 
            "x_adder.np14_7", 
            false, 
            Some(format!("td+clkper*{}", i*2+1)), 
            1, 
            true, 
            Some(format!("td+clkper*{}", i*2+1)), 
            1,
        );
    }
    println!("{}", s);
}


#[test]
fn test_measure_delay_standard() {
    let mut content = String::new();
    for a in 0..31 {
        for s in a..31 {
            content += &line_measure_delay_with_td(
                &format!("base_1_A{a:02}_to_S{s:02}_f"), 
                &format!("S{s:02}"), 
                &format!("A{a:02}"), 
                true, 
                Some(format!("td+clkper*{}", a*2)), 
                1, 
                true, 
                Some(format!("td+clkper*{}", a*2)), 
                1,
            );
        }
    }
    use std::fs::File;
    use std::io::prelude::*;
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(content.as_bytes());
}
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
use crate::hspice::{adder_call_std, line_inc, line_measure_delay, line_source};

pub fn create_sp_of_adder_timing_rise(
    adder_name : &str,
    adder_cdl_path : &str,
    bits : usize,
) -> String {
    let mut txt = String::new();
    txt += &line_inc(adder_cdl_path);
    txt += "\n";

    // source
    for i in 0..bits {
        txt += &line_source(&format!("A{i}"), "0", "avdd", 2. * i as f64, 1., None)
    }
    txt += "\n";

    // adder
    txt += &adder_call_std(bits, adder_name);
    txt += "\n";

    // measure
    let target_wire = "S30";
    for i in 0..bits {
        let source_wire = format!("A{i}");
        txt += &line_measure_delay(
            &format!("{source_wire}_to_{target_wire}"),
            target_wire, 
            &source_wire, 
            true, 
            1, 
            true, 
            i+1
        );
    }

    txt
}
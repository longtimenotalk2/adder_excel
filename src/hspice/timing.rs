use crate::{cell_parse::ProcessAndProject, hspice::{line_inc, line_measure_delay, line_measure_delay_with_td, line_source_dc, line_source_period}, std::adder::Adder};

impl Adder {
    pub fn create_sp_of_adder_timing_base_0(
        &self,
        process : ProcessAndProject,
        adder_name : &str,
        adder_cdl_path : &str,
    ) -> String {
        self.create_sp_of_adder_timing_base_0_sp(
            process,
            adder_name,
            adder_cdl_path,
            true,
            true
        )
    }
    pub fn create_sp_of_adder_timing_base_0_sp(
        &self,
        process : ProcessAndProject,
        adder_name : &str,
        adder_cdl_path : &str,
        r2f : bool,
        f2r : bool,
    ) -> String {
        let mut txt = String::new();
        let bits = self.bits;
        txt += &line_inc(adder_cdl_path);
        txt += "\n";

        // source
        for i in 0..bits {
            txt += &line_source_period(&format!("A{i:02}"), "0", "avdd", 2. * i as f64, 1., None)
        }
        for i in 0..bits {
            txt += &line_source_dc(&format!("B{i:02}"), "avdd")
        }
        txt += "\n";

        // adder
        txt += &self.adder_call(process, adder_name);
        txt += "\n";

        // measure
        for i in 0..bits {
            let source_wire = format!("A{i:02}");
            for j in i..bits {
                let target_wire = format!("S{j:02}");
                if r2f {
                    txt += &line_measure_delay_with_td(
                        &format!("base_0_{source_wire}_r_to_{target_wire}_f"),
                        &target_wire, 
                        &source_wire, 
                        true, 
                        None,
                        1, 
                        false, 
                        Some(format!("'td+clkper*{}'", 2*i)),
                        1
                    );
                }
                if f2r {
                    txt += &line_measure_delay_with_td(
                        &format!("base_0_{source_wire}_f_to_{target_wire}_r"),
                        &target_wire, 
                        &source_wire, 
                        false, 
                        None,
                        1, 
                        true, 
                        Some(format!("'td+clkper*{}'", 2*i+1)),
                        1
                    );
                }
            }
            
        }
        txt += "\n";


        // pg
        txt += &line_source_dc("VBB", "0");
        txt += &line_source_dc("VDD", "avdd");
        txt += &line_source_dc("VPP", "avdd");
        txt += &line_source_dc("VSS", "0");
        txt += "\n";


        txt
    }

    pub fn create_sp_of_adder_timing_base_1_sp(
        &self,
        process : ProcessAndProject,
        adder_name : &str,
        adder_cdl_path : &str,
        r2f : bool,
        f2r : bool,
    ) -> String {
        let mut txt = String::new();
        let bits = self.bits;
        txt += &line_inc(adder_cdl_path);
        txt += "\n";

        // source
        for i in 0..bits {
            txt += &line_source_period(&format!("A{i:02}"), "avdd", "0", 2. * i as f64, 1., None)
        }
        for i in 0..bits {
            txt += &line_source_dc(&format!("B{i:02}"), if i == 0 { "avdd" } else { "0" })
        }
        txt += "\n";

        // adder
        txt += &self.adder_call(process, adder_name);
        txt += "\n";

        // measure
        for i in 0..bits {
            let source_wire = format!("A{i:02}");
            for j in i..bits {
                let target_wire = format!("S{j:02}");
                if r2f {
                    txt += &line_measure_delay_with_td(
                        &format!("base_1_{source_wire}_r_to_{target_wire}_f"),
                        &target_wire, 
                        &source_wire, 
                        true, 
                        None,
                        1, 
                        false, 
                        Some(format!("'td+clkper*{}'", 2*i+1)),
                        1
                    );
                }
                if f2r {
                    txt += &line_measure_delay_with_td(
                        &format!("base_1_{source_wire}_f_to_{target_wire}_r"),
                        &target_wire, 
                        &source_wire, 
                        false, 
                        None,
                        1, 
                        true, 
                        Some(format!("'td+clkper*{}'", 2*i)),
                        1
                    );
                }
            }
            
        }
        txt += "\n";


        // pg
        txt += &line_source_dc("VBB", "0");
        txt += &line_source_dc("VDD", "avdd");
        txt += &line_source_dc("VPP", "avdd");
        txt += &line_source_dc("VSS", "0");
        txt += "\n";


        txt
    }
}


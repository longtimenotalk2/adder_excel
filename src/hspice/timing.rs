use crate::{cell_parse::{ProcessAndProject, RealCell}, hspice::{line_cell, line_inc, line_measure_delay, line_measure_delay_with_td, line_source_dc, line_source_period}, std::{adder::{AbstractCell, Adder, Drive}, logic_block::LogicBlock}};

impl Adder {
    pub fn create_sp_of_adder_timing_single(
        &self,
        process : ProcessAndProject,
        adder_name : &str,
        adder_cdl_path : &str,
        index : usize,
    ) -> String {
        let mut txt = String::new();
        let bits = self.bits;
        txt += &line_inc(adder_cdl_path);
        txt += "\n";

        // source
        for i in 0..bits {
            if i == index {
                txt += &line_source_period(&format!("A{i:02}"), "0", "avdd", 0., 1., None)
            } else {
                txt += &line_source_dc(&format!("A{i:02}"), "0")
            }
        }
        for i in 0..bits {
            txt += &line_source_dc(&format!("B{i:02}"), "avdd")
        }
        txt += "\n";

        // adder
        txt += &self.adder_call(process, adder_name);
        txt += "\n";


        // pg
        txt += &line_source_dc("VBB", "0");
        txt += &line_source_dc("VDD", "avdd");
        txt += &line_source_dc("VPP", "avdd");
        txt += &line_source_dc("VSS", "0");
        txt += "\n";


        txt
    }



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
                        self.input_is_neg ^ self.output_is_neg, 
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
                        !self.input_is_neg ^ self.output_is_neg, 
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

    pub fn create_sp_of_adder_timing_base_1(
        &self,
        process : ProcessAndProject,
        adder_name : &str,
        adder_cdl_path : &str,
    ) -> String {
        self.create_sp_of_adder_timing_base_1_sp(
            process,
            adder_name,
            adder_cdl_path,
            true,
            true
        )
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
                        self.input_is_neg ^ self.output_is_neg, 
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
                        !self.input_is_neg ^ self.output_is_neg, 
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


// 新型生成timing
/* 需求
base0和base1
是否需要前垫两个INV
前序测量与否
后续测量与否
*/
impl Adder {
    pub fn timing_all_pre_two_inv(
        &self,
        process : ProcessAndProject,
        adder_name : &str,
        adder_cdl_path : &str,
        is_base1 : bool, // base0是，A全0，B全1，A逐个上升再下降。base1是，A全1，B全0（第0bit是1），A逐个下降再上升
        is_measure_start_r : bool,
        is_measure_start_f : bool,
    ) -> String {
        let mut txt = String::new();
        let bits = self.bits;
        

        let cell_inv = RealCell::parse(process, &AbstractCell {logic_block:LogicBlock::INV, drive:Drive::D1,custom_demand:vec![]}).name;

        // source
        for i in 0..bits {
            if is_base1 {
                txt += &line_source_period(&format!("A{i:02}PP"), "avdd", "0", 2. * i as f64, 1., None)
            } else {
                txt += &line_source_period(&format!("A{i:02}PP"), "0", "avdd", 2. * i as f64, 1., None)
            }
            
        }
        for i in 0..bits {
            if is_base1 {
                txt += &line_source_dc(&format!("B{i:02}PP"), if i == 0 { "avdd" } else { "0" })
            } else {
                txt += &line_source_dc(&format!("B{i:02}PP"), "avdd")
            }
        }
        txt += "\n";
        for i in 0..bits {
            txt += &line_cell(&format!("PRE_A{i:02}_P"), &[format!("A{i:02}PP"), "VBB".to_string(), "VDDP".to_string(), "VPP".to_string(), "VSS".to_string(), format!("A{i:02}P")], &cell_inv);
            txt += &line_cell(&format!("PRE_A{i:02}"), &[format!("A{i:02}P"), "VBB".to_string(), "VDDP".to_string(), "VPP".to_string(), "VSS".to_string(), format!("A{i:02}")], &cell_inv);
            txt += &line_cell(&format!("PRE_B{i:02}_P"), &[format!("B{i:02}PP"), "VBB".to_string(), "VDDP".to_string(), "VPP".to_string(), "VSS".to_string(), format!("B{i:02}P")], &cell_inv);
            txt += &line_cell(&format!("PRE_B{i:02}"), &[format!("B{i:02}P"), "VBB".to_string(), "VDDP".to_string(), "VPP".to_string(), "VSS".to_string(), format!("B{i:02}")], &cell_inv);
        }
        txt += "\n";

        // adder
        txt += &line_inc(adder_cdl_path);
        txt += "\n";

        txt += &self.adder_call(process, adder_name);
        txt += "\n";

        // measure
        let base_index = if is_base1 { 1 } else { 0 };
        for i in 0..bits {
            let source_wire = format!("A{i:02}");
            for j in i..bits {
                let target_wire = format!("S{j:02}");
                if is_measure_start_r {
                    txt += &line_measure_delay_with_td(
                        &format!("base_{base_index}_{source_wire}_r_to_{target_wire}_f"),
                        &target_wire, 
                        &source_wire, 
                        true, 
                        None,
                        1, 
                        self.input_is_neg ^ self.output_is_neg, 
                        Some(format!("'td+clkper*{}'", 2*i+base_index)),
                        1
                    );
                }
                if is_measure_start_f {
                    txt += &line_measure_delay_with_td(
                        &format!("base_{base_index}_{source_wire}_f_to_{target_wire}_r"),
                        &target_wire, 
                        &source_wire, 
                        false, 
                        None,
                        1, 
                        !self.input_is_neg ^ self.output_is_neg, 
                        Some(format!("'td+clkper*{}'", 2*i+1-base_index)),
                        1
                    );
                }
            }
            
        }
        txt += "\n";


        // pg
        txt += &line_source_dc("VBB", "0");
        txt += &line_source_dc("VDD", "avdd");
        txt += &line_source_dc("VDDP", "avdd");
        txt += &line_source_dc("VPP", "avdd");
        txt += &line_source_dc("VSS", "0");
        txt += "\n";


        txt
    }
}
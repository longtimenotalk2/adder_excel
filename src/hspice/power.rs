use crate::std::adder::Adder;

use super::*;

impl Adder {
    pub fn create_sp_of_adder_power(
        &self,
        process : ProcessAndProject,
        adder_name : &str,
        adder_cdl_path : &str,
    ) -> String {
        let mut txt = String::new();
        let bits = self.bits;
        txt += &line_inc(adder_cdl_path);
        txt += "\n";

        // source
        let mut seed = 0;
        for i in 0..bits {
            txt += &line_source_random(&format!("A{i:02}"), seed, "0", "avdd");
            seed += 1;
        }
        for i in 0..bits {
            txt += &line_source_random(&format!("B{i:02}"), seed, "0", "avdd");
            seed += 1;
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

        // measure
        txt += &line_measure_power("VDD");


        txt
    }

    pub fn create_sp_of_adder_power_2nd(
        &self,
        process : ProcessAndProject,
        adder_name : &str,
        adder_cdl_path : &str,
        left_shift : usize,
    ) -> String {
        let mut txt = String::new();
        let bits = self.bits;
        txt += &line_inc(adder_cdl_path);
        txt += "\n";

        // source
        let mut seed = 114514;
        for i in 0..bits {
            txt += &line_source_random(&format!("C{i:02}"), seed, "0", "avdd");
            seed += 1;
        }

        txt += "\n";

        // adder
        let mut a_s = vec![];
        let mut b_s = vec![];
        let mut s_s = vec![];

        for i in 0..bits {
            a_s.push(format!("S{:02}", (i + left_shift) % self.bits));
            b_s.push(format!("C{i:02}"));
            s_s.push(format!("T{i:02}"));
        }
        txt += &self.adder_call_with(process, adder_name, &a_s, &b_s, &s_s);
        txt += "\n";


        // pg
        txt += &line_source_dc("VBB", "0");
        txt += &line_source_dc("VDD", "avdd");
        txt += &line_source_dc("VPP", "avdd");
        txt += &line_source_dc("VSS", "0");
        txt += "\n";

        // measure
        txt += &line_measure_power("VDD");


        txt
    }

    pub fn create_sp_of_adder_power_all_vdd_split(&self,
        process : ProcessAndProject,
        adder_name : &str,
        adder_cdl_path : &str,
    ) -> String {
        let mut txt = String::new();
        let bits = self.bits;
        txt += &line_inc(adder_cdl_path);
        txt += "\n";

        // source
        let mut seed = 0;
        for i in 0..bits {
            txt += &line_source_random(&format!("A{i:02}"), seed, "0", "avdd");
            seed += 1;
        }
        for i in 0..bits {
            txt += &line_source_random(&format!("B{i:02}"), seed, "0", "avdd");
            seed += 1;
        }

        txt += "\n";

        // adder
        txt += &self.adder_call_all_vdd_split(process, adder_name);
        txt += "\n";


        // pg
        txt += &line_source_dc("VBB", "0");
        for cell in &self.cells {
            txt += &line_source_dc(&format!("VDD_{}", cell.inst_name()), "avdd");
        }
        txt += &line_source_dc("VPP", "avdd");
        txt += &line_source_dc("VSS", "0");
        txt += "\n";

        // measure
        for cell in &self.cells {
            txt += &line_measure_power(&format!("VDD_{}", cell.inst_name()));
        }


        txt
    }

    pub fn create_sp_source_trival(&self) -> String {
        let mut txt = String::new();
        // source
        txt += &line_source_period(&format!("A00"), "0", "avdd", 0., 2., Some(4.));
        for i in 1..self.bits {
            txt += &line_source_dc(&format!("A{i:02}"), "0")
        }
        for i in 0..self.bits {
            txt += &line_source_dc(&format!("B{i:02}"), "avdd")
        }
        txt += "\n";
        txt
    }
}
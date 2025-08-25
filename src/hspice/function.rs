use crate::std::adder::Adder;

use super::*;

impl Adder {
    pub fn create_sp_of_adder_function(
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


        txt
    }
}
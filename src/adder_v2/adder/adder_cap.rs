use colorful::{Color, Colorful};

use crate::adder_v2::{adder::Adder, excel::{excel_to_datalist::ExcelDataList, ExcelFrame, ExcelKey}, wire::Wire, Id};

impl Adder {
    pub fn get_cap_cmos_for_wire(&self, wire: &(Id, Wire)) -> i32 {
        let mut cap = 0;

        for (_, cell) in self.cells.iter() {
            // O1 cap
            if let Some(wire_o1) = &cell.node.io.output_o1 {
                if wire_o1 == wire {
                    let cell_body = cell.to_cell_body();
                    cap += cell_body.cap_cmos_o1_inner();
                }  
            }
            // load
            for (port, w) in cell.node.io.input.iter() {
                if w == wire {
                    cap += cell.to_cell_body().cap_cmos_given_input_port(port);
                }
            }
        }

        cap
    }

    pub fn get_all_cap_by_excel(&self, excel_frame : &ExcelFrame, excel_map : &ExcelDataList<Id>) {
        

        let mut excel_frame_new = excel_frame.clone();
        let mut mismatch_count = 0;
        for (excel_key, cell_id) in excel_map.data.iter() {
            let multi_line_id = excel_key.multi_line_id;
            let excel_index = excel_key.index;
            let excel_multi_line = &excel_frame.multi_lines.get(&ExcelKey {multi_line_id}).unwrap();
            let cap_in_excel = &excel_multi_line.cap_line[excel_index];
            let excel_caps = if cap_in_excel.trim().len() > 0 {
                cap_in_excel.split(",").map(|txt| txt.trim().parse::<i32>().expect(
                    &format!("cap {txt} can not parse i32, at layer {}, {}, index {excel_index}", 
                            excel_multi_line.layer, excel_multi_line.name)
                )).collect::<Vec<i32>>()
            } else {vec![0]};
            
            let wire_z = &self.cells.get(*cell_id as usize).unwrap().1.node.io.output_z;
            let real_cap_z = self.get_cap_cmos_for_wire(wire_z);
            let wire_o1 = &self.cells.get(*cell_id as usize).unwrap().1.node.io.output_o1;
            let real_cap_o1 : Option<i32> = wire_o1.as_ref().map(|w| self.get_cap_cmos_for_wire(&w));


            let real_caps = if let Some(wire_o1) = wire_o1 {
                let real_cap_o1 = real_cap_o1.unwrap();
                vec![real_cap_o1, real_cap_z]
            } else {
                vec![real_cap_z]
            };

            if excel_caps != real_caps {
                println!("> {:03} : cell {} cap not match : {} != {}", 
                    cell_id,
                    self.cells.get(*cell_id as usize).unwrap().1.to_string(),
                    format!("{:?}", real_caps).color(Color::Green).to_string(),
                    format!("{:?}", excel_caps).color(Color::Red).to_string());
                mismatch_count += 1;
            }
            let cap_txt_new = real_caps.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(",");
            excel_frame_new.multi_lines.get_mut(&ExcelKey {multi_line_id}).unwrap().cap_line[excel_index] = cap_txt_new;
            
        }

        if mismatch_count == 0 {
            print!(">>> check cap with excel data ...  ");
            println!("{}", "pass !".to_string().color(Color::Green));
        } else {
            print!(">>> check cap with excel data ...  ");
            println!("{}", "fail !".to_string().color(Color::Red));
        }

        let mut real_cap_lines = String::new();
        for (_, data) in excel_frame_new.multi_lines {
            real_cap_lines += format!("{}\t{}\tcap", data.layer, data.name).as_str();
            for index in (0..self.bits).rev() {
                real_cap_lines += format!("\t{}", data.cap_line[index]).as_str();
            }
            real_cap_lines += "\n";
        }

        use std::fs::File;
        use std::io::prelude::*;
        let content = "This is the content to write to the file.";
        // 创建一个新文件，如果文件已存在，则覆盖
        let mut file = File::create("output.txt").unwrap();
        // 将字符串写入文件
        let _ = file.write_all(real_cap_lines.as_bytes());
    }
}
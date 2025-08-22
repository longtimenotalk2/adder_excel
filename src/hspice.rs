use std::collections::BTreeMap;

use crate::{cell_parse::{CellSourceType, ProcessAndProject, RealCell}, std::node_create::LogicBlockMappingTable};

pub mod cdl;
pub mod timing;

const SPACE : &str = "    ";

fn line_inc(path : &str) -> String {
    format!(".inc '{path}'\n")
}

fn line_subckt(inst_logo : &str, pins : &[impl ToString], ) -> String {
    let mut s = String::new();
    s += ".SUBCKT ";
    s += inst_logo;
    s += " ";
    for pin in pins {
        s += &format!("{} ", pin.to_string());
    };
    s += "\n";
    s
}

pub fn line_cell(inst_name : &str, pins : &[impl ToString], inst_logo : &str) -> String {
    let mut s = String::new();
    s += &format!("{:<16}{SPACE}",
        format!("x_{inst_name}")
    );
    for pin in pins {
        s += &format!("{:<4}{SPACE}", pin.to_string());
    }
    s += &format!("{inst_logo}\n");
    s
}

pub fn line_cell_given_lens(inst_name : &str, pins : &[impl ToString], inst_logo : &str, given_lens : &[usize]) -> String {
    let mut s = String::new();
    s += &format!("x_{inst_name}\n");
    let mut index = 0;
    for len in given_lens {
        s += "+ ";
        for _ in 0..*len {
            s += &format!("{:<4}{SPACE}", pins[index].to_string());
            index += 1;
        }
        s += "\n";
    }
    s += &format!("+ {inst_logo}\n");
    s
}

pub fn line_source(wire_name : &str, v1 : &str, v2 : &str, start_pos : f64, step_length : f64, period : Option<f64>) -> String {
    format!("V{wire_name}  {wire_name}  0  pulse  {v1}  {v2}  'td+clkper*{start_pos}'  tr  tr  'clkper*{step_length}'  '{}'\n",  
        if let Some(period) = period {
            format!("clkper*{period}")
        } else {
            "time_all".to_string()
        }
    )
}

pub fn line_end_subckt() -> String {
    ".ENDS".to_string()
}

pub fn line_measure_delay(
    name : &str,
    target_wire : &str, 
    source_wire : &str, 
    source_is_rise : bool,
    source_nth : usize,
    target_is_rise : bool,
    target_nth : usize,
) -> String {
    format!(".measure tran delay_{name}    trig v({source_wire}) val='avdd/2'   {}={source_nth}      targ v({target_wire}) val='avdd/2'   {}={target_nth}\n",
        if source_is_rise { "rise" } else { "fall" },
        if target_is_rise { "rise" } else { "fall" }
    )
}

impl RealCell {
    fn line_inc(&self) -> String {
        let path = match self.process {
            ProcessAndProject::N3E1374 => todo!(),
            ProcessAndProject::N4C1340 => {
                let path_base = "/ic/projects/BM1340";
                let end = ".Cbest60.spf";
                match self.source_type {
                    CellSourceType::Std => format!("{path_base}/public/5_custom/spf/stdcell/Cbest/{}{end}", self.name),
                    CellSourceType::Custom => format!("{path_base}/public/5_custom/spf/custom/Cbest/{}{end}", self.name),
                    CellSourceType::Lhw => todo!()
                }
            },
        };
        line_inc(&path)
    }

    fn line_cell(&self, inst_name : &str, map : &LogicBlockMappingTable) -> String {
        let pg_port = ["VBB", "VDD", "VPP", "VSS"];
        let mut port_wire_map = BTreeMap::new();
        for port in pg_port {
            port_wire_map.insert(port.to_string(), port.to_string());
        }
        for (port, wire) in &map.inputs {
            port_wire_map.insert(port.0.clone(), wire.to_string());
        }
        for (port, wire) in &map.outputs {
            port_wire_map.insert(port.0.clone(), wire.to_string());
        }
        let pins : Vec<String> = port_wire_map.values().cloned().collect();

        line_cell(inst_name, &pins, &self.name)
    }
}

fn adder_pins_std(bits : usize) -> Vec<String> {
    let mut ports = vec![];
    for i in 0..bits {
        ports.push(format!("A{i}"));
    }
    for i in 0..bits {
        ports.push(format!("B{i}"));
    }
    for i in 0..bits {
        ports.push(format!("S{i}"));
    }
    ports.append(&mut vec!["VBB".to_string(), "VDD".to_string(), "VPP".to_string(), "VSS".to_string()]);
    ports
}

fn adder_call_std(bits : usize, inst_name : &str) -> String {
    let mut lens = vec![];
    let mut remains = bits;
    while remains >= 8 {
        lens.push(8);
        remains -= 8;
    }
    lens.push(remains);
    lens = [lens.clone(), lens.clone(), lens.clone(), vec![4]].concat();
    line_cell_given_lens("ADDER", &adder_pins_std(bits), inst_name, &lens)
}
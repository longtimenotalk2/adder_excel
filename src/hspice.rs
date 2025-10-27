use std::collections::BTreeMap;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::{cell_parse::{CellSourceType, ProcessAndProject, RealCell}, std::node_create::LogicBlockMappingTable};

pub mod cdl;
pub mod timing;
pub mod function;
pub mod power;
mod other;
pub mod test_out;

const SPACE : &str = "    ";

pub fn line_inc(path : &str) -> String {
    format!(".inc '{path}'\n")
}

pub fn line_subckt(inst_logo : &str, pins : &[impl ToString], ) -> String {
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

pub fn line_source_period(wire_name : &str, v1 : &str, v2 : &str, start_pos : f64, step_length : f64, period : Option<f64>) -> String {
    format!("V{wire_name}  {wire_name}  0  pulse  {v1}  {v2}  'td+clkper*{start_pos}'  tr  tr  'clkper*{step_length}-tr'  '{}'\n",  
        if let Some(period) = period {
            format!("clkper*{period}")
        } else {
            "time_all".to_string()
        }
    )
}

pub fn line_cap(pinname : &str, cap : &str) -> String {
    format!("C{pinname} {pinname} 0 {cap}\n")
}

pub fn line_source_dc(pinname : &str, source_name : &str) -> String {
    let mut s = String::new();
    s += &format!("{:<16}{SPACE}{:<4}{SPACE}0   DC={source_name}\n",
        format!("V{pinname}"),
        pinname,
    );
    s
}

pub fn line_source_random(pinname : &str, seed : u64, source_0 : &str, source_1 : &str) -> String {
    let backs = [32, 22, 2, 1, 0];
    let mut my_rng = ChaCha20Rng::seed_from_u64(seed);
    let my_seed : u32 = my_rng.random(); 
    let mut s_last = String::new();
    for b in backs {
        s_last += &format!("{b},");
    }
    s_last = s_last[..s_last.len()-1].to_string();

    format!("V{pinname} {pinname} 0 LFSR ({source_0} {source_1} 'td-clkper+tr'  tr tr (1/clkper) {} [{s_last}])\n", my_seed)
}

pub fn line_end_subckt() -> String {
    ".ENDS\n".to_string()
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

pub fn line_measure_delay_with_given_pg(
    name : &str,
    target_wire : &str, 
    source_wire : &str, 
    source_is_rise : bool,
    source_nth : usize,
    source_pg : &str,
    target_is_rise : bool,
    target_nth : usize,
    target_pg : &str,
) -> String {
    format!(".measure tran delay_{name}    trig v({source_wire}) val='{source_pg}'   {}={source_nth}      targ v({target_wire}) val='{target_pg}'   {}={target_nth}\n",
        if source_is_rise { "rise" } else { "fall" },
        if target_is_rise { "rise" } else { "fall" }
    )
}

pub fn line_measure_delay_with_td(
    name : &str,
    target_wire : &str, 
    source_wire : &str, 
    source_is_rise : bool,
    source_td : Option<String>,
    source_nth : usize,
    target_is_rise : bool,
    target_td : Option<String>,
    target_nth : usize,
) -> String {
    format!(".measure tran delay_{name}    trig v({source_wire}) val='avdd/2' {}  {}={source_nth}      targ v({target_wire}) val='avdd/2'   {}  {}={target_nth}\n",
        if let Some(td) = source_td { format!("td={td}") } else { "".to_string() },
        if source_is_rise { "rise" } else { "fall" },
        if let Some(td) = target_td { format!("td={td}") } else { "".to_string() },
        if target_is_rise { "rise" } else { "fall" }
    )
}

pub fn line_measure_power(pinname : &str) -> String {
    let mut s = String::new();
    s += &format!(".measure Tran power_{pinname}    avg i(V{pinname})   from='td' to='time_all'\n");
    s
}

impl RealCell {
    pub fn inc_path(&self) -> String {
        let path = match self.process {
            ProcessAndProject::N3E1374 => {
                let path_base = "/ic/projects/BM1374";
                let end = ".Cbest45.spf";
                match &self.source_type {
                    CellSourceType::Std => format!("{path_base}/public/5_custom/release/stdcell/stdcell_BM/elvt/spf/Cbest45/{}{end}", self.name),
                    CellSourceType::Custom => format!("{path_base}/public/5_custom/release/custom/elvt/spf/Cbest45/{}{end}", self.name),
                    CellSourceType::Lhw => format!("{path_base}/users/haiwei.li/V0/work/spf/out/{}/{}{end}", self.name, self.name),
                    CellSourceType::LocalHack => format!("cell/hack_{}{end}", self.name),
                }
            },
            ProcessAndProject::N4C1340 => {
                let path_base = "/ic/projects/BM1340";
                let end = ".Cbest60.spf";
                match &self.source_type {
                    CellSourceType::Std => format!("{path_base}/public/5_custom/spf/stdcell/Cbest/{}{end}", self.name),
                    CellSourceType::Custom => format!("{path_base}/public/5_custom/spf/custom/Cbest/{}{end}", self.name),
                    CellSourceType::Lhw => format!("{path_base}/users/haiwei.li/V0/work/spf/out/{}/{}{end}", self.name, self.name),
                    CellSourceType::LocalHack => format!("cell/hack_{}{end}", self.name),
                }
            },
            ProcessAndProject::N4C1342H200 => {
                todo!()
            }
        };
        path
    }

    fn line_inc(&self) -> String {
        
        line_inc(&self.inc_path())
    }

    fn line_cell(&self, inst_name : &str, map : &LogicBlockMappingTable) -> String {
        let mut pg_port = vec!["VBB", "VDD", "VPP", "VSS"];
        if self.vdd_replaced.len() > 0 {
            pg_port = vec!["VBB"];
            for port in &self.vdd_replaced {
                pg_port.push(port.0.as_str());
            }
            pg_port.push("VPP");
            pg_port.push("VSS");
        }
        for port in &self.addition_pg_port {
            pg_port.push(port.0.as_str());
        }
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

    fn line_cell_vdd_split(&self, inst_name : &str, map : &LogicBlockMappingTable) -> String {
        let mut pg_port = vec!["VBB".to_string(), format!("VDD_{}", inst_name), "VPP".to_string(), "VSS".to_string()];
        for port in &self.addition_pg_port {
            pg_port.push(port.0.clone());
        }
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

fn adder_pins_std_two_index(bits : usize) -> Vec<String> {
    let mut ports = vec![];
    for i in 0..bits {
        ports.push(format!("A{i:02}"));
    }
    for i in 0..bits {
        ports.push(format!("B{i:02}"));
    }
    for i in 0..bits {
        ports.push(format!("S{i:02}"));
    }
    ports.append(&mut vec!["VBB".to_string(), "VDD".to_string(), "VPP".to_string(), "VSS".to_string()]);
    ports
}

// fn adder_pins_std(bits : usize) -> Vec<String> {
//     let mut ports = vec![];
//     for i in 0..bits {
//         ports.push(format!("A{i}"));
//     }
//     for i in 0..bits {
//         ports.push(format!("B{i}"));
//     }
//     for i in 0..bits {
//         ports.push(format!("S{i}"));
//     }
//     ports.append(&mut vec!["VBB".to_string(), "VDD".to_string(), "VPP".to_string(), "VSS".to_string()]);
//     ports
// }

// fn adder_call_std(bits : usize, inst_name : &str) -> String {
//     let mut lens = vec![];
//     let mut remains = bits;
//     while remains >= 8 {
//         lens.push(8);
//         remains -= 8;
//     }
//     lens.push(remains);
//     lens = [lens.clone(), lens.clone(), lens.clone(), vec![4]].concat();
//     line_cell_given_lens("ADDER", &adder_pins_std_two_index(bits), inst_name, &lens)
// }
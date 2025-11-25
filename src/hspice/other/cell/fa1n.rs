use crate::hspice::{line_cell, line_measure_delay, line_measure_power, line_source_dc};

use super::*;

const COVER : &'static str = "cover_FA1N";

fn sn_logic(a : bool, b : bool, ci : bool) -> bool {
    !a ^ b ^ ci
}

fn sn_out_signal(input_arc : &InputArc) -> Signal {
    let start = input_arc.start();
    let end = input_arc.end();
    let start_out = sn_logic(start[0], start[1], start[2]);
    let end_out = sn_logic(end[0], end[1], end[2]);
    Signal::from_start_and_end(start_out, end_out)
}

fn con_logic(a : bool, b : bool, ci : bool) -> bool {
    !((a&b)|(b&ci)|(ci&a))
}

fn con_out_signal(input_arc : &InputArc) -> Signal {
    let start = input_arc.start();
    let end = input_arc.end();
    let start_out = con_logic(start[0], start[1], start[2]);
    let end_out = con_logic(end[0], end[1], end[2]);
    Signal::from_start_and_end(start_out, end_out)
}

fn fa1n_input_arcs() -> Vec<InputArc> {
    super::get_all_input_arcs(3, 1)
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Fa1nArcType {
    A00F,
    A00R,
    A01F,
    A01R,
    A11F,
    A11R,
}

impl Fa1nArcType {
    fn from_str(s: &str) -> Self {
        match s {
            "00f" => Self::A00F,
            "00r" => Self::A00R,
            "01f" => Self::A01F,
            "01r" => Self::A01R,
            "11f" => Self::A11F,
            "11r" => Self::A11R,
            _ => panic!("Unknown arc type: {}", s),
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            Self::A00F => "00f",
            Self::A00R => "00r",
            Self::A01F => "01f",
            Self::A01R => "01r",
            Self::A11F => "11f",
            Self::A11R => "11r",
        }
    }
}

impl Debug for Fa1nArcType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::A00F => "00f",
            Self::A00R => "00r",
            Self::A01F => "01f",
            Self::A01R => "01r",
            Self::A11F => "11f",
            Self::A11R => "11r",
        };
        write!(f, "{}", s)
    }
}

fn sorted_fa1n_input_arcs_with_type() -> Vec<(Fa1nArcType, InputArc)> {
    let arcs = fa1n_input_arcs();
    let mut type_and_arcs = Vec::new();
    for arc in arcs {
        let mut sorted_signals = arc.0.clone();
        sorted_signals.sort();
        let type_name = format!("{:?}", InputArc(sorted_signals));
        let type_arc = Fa1nArcType::from_str(&type_name);
        type_and_arcs.push((type_arc, arc));
    }
    type_and_arcs.sort();
    type_and_arcs
}

fn timing_arc_measure_for_sn() -> String {
    let mut txt = String::new();

    for (type_arc, input_arc) in sorted_fa1n_input_arcs_with_type() {
        let sn_out = sn_out_signal(&input_arc);
        txt += &format!("*---- arc : <{:?}> {:?} -> {:?} ----\n", type_arc, input_arc, sn_out);

        let target_wire_sn = format!("q_SN_type{}_{}", type_arc.to_str(), input_arc.to_string());
        txt += &line_cell(&input_arc.to_string(), &[
            input_arc.0[0].to_wire(),
            input_arc.0[1].to_wire(),
            input_arc.0[2].to_wire(),
            "VBB",
            "VPP",
            "VSS",
            "VDD",
            "VDD",
            "VDD",
            format!("d_A_type{}_{}", type_arc.to_str(), input_arc.to_string()).as_str(),
            format!("d_B_type{}_{}", type_arc.to_str(), input_arc.to_string()).as_str(),
            format!("d_CI_type{}_{}", type_arc.to_str(), input_arc.to_string()).as_str(),
            format!("z_CON_type{}_{}", type_arc.to_str(), input_arc.to_string()).as_str(),
            format!("z_SN_type{}_{}", type_arc.to_str(), input_arc.to_string()).as_str(),
            format!("q_CON_type{}_{}", type_arc.to_str(), input_arc.to_string()).as_str(),
            target_wire_sn.as_str(),
        ], COVER);
        // measure delay
        txt += &line_measure_delay(
            format!("sn_{}", input_arc.to_string()).as_str(), 
            target_wire_sn.as_str(), 
            RISE, 
            true, 
            1, 
            match sn_out {
                Signal::Rise => true,
                Signal::Fall => false,
                _ => panic!(),
            }, 
            1
        );
    }

    txt
}

fn timing_arc_measure_for_con_attend() -> String {
    let mut txt = String::new();

    for (type_arc, input_arc) in sorted_fa1n_input_arcs_with_type() {
        let con_out = con_out_signal(&input_arc);
        if con_out.is_flip() {
            txt += &format!("*---- arc_con : <{:?}> {:?} -> {:?} ----\n", type_arc, input_arc, con_out);

            let target_wire_sn = format!("q_CON_type{}_{}", type_arc.to_str(), input_arc.to_string());
            // txt += &line_cell(&input_arc.to_string(), &[
            //     input_arc.0[0].to_wire(),
            //     input_arc.0[1].to_wire(),
            //     input_arc.0[2].to_wire(),
            //     "VBB",
            //     "VPP",
            //     "VSS",
            //     "VDD",
            //     "VDD",
            //     "VDD",
            //     format!("d_A_type{}_{}", type_arc.to_str(), input_arc.to_string()).as_str(),
            //     format!("d_B_type{}_{}", type_arc.to_str(), input_arc.to_string()).as_str(),
            //     format!("d_CI_type{}_{}", type_arc.to_str(), input_arc.to_string()).as_str(),
            //     format!("z_CON_type{}_{}", type_arc.to_str(), input_arc.to_string()).as_str(),
            //     format!("z_SN_type{}_{}", type_arc.to_str(), input_arc.to_string()).as_str(),
            //     format!("q_CON_type{}_{}", type_arc.to_str(), input_arc.to_string()).as_str(),
            //     target_wire_sn.as_str(),
            // ], COVER);
            // measure delay
            txt += &line_measure_delay(
                format!("con_{}", input_arc.to_string()).as_str(), 
                target_wire_sn.as_str(), 
                RISE, 
                true, 
                1, 
                match con_out {
                    Signal::Rise => true,
                    Signal::Fall => false,
                    _ => panic!(),
                }, 
                1
            );
        }
        
    }

    txt
}

fn leakage() -> String {
    let mut txt = String::new();

    let input_arcs = super::get_all_input_arcs(3, 0);

    for input_arc in input_arcs {
        let sn_out = sn_out_signal(&input_arc);
        txt += &format!("*---- arc : {:?} -> {:?} ----\n", input_arc, sn_out);

        let target_vdd = format!("VDD_MAIN_{}", input_arc.to_string());
        txt += &line_source_dc(&target_vdd, "avdd");
        txt += &line_cell(&input_arc.to_string(), &[
            input_arc.0[0].to_wire(),
            input_arc.0[1].to_wire(),
            input_arc.0[2].to_wire(),
            "VBB",
            "VPP",
            "VSS",
            "VDD",
            target_vdd.as_str(),
            "VDD",
            format!("d_A_{}", input_arc.to_string()).as_str(),
            format!("d_B_{}", input_arc.to_string()).as_str(),
            format!("d_CI_{}", input_arc.to_string()).as_str(),
            format!("z_CON_{}", input_arc.to_string()).as_str(),
            format!("z_SN_{}", input_arc.to_string()).as_str(),
            format!("q_CON_{}", input_arc.to_string()).as_str(),
            format!("q_SN_{}", input_arc.to_string()).as_str(),
        ], COVER);
        txt += &line_measure_power(&target_vdd);
    }

    txt
}

fn dynamic() -> String {
    let mut txt = String::new();

    let mut input_arcs = super::get_all_input_arcs(3, 0);
    input_arcs.append(&mut super::get_all_input_arcs(3, 1));
    input_arcs.append(&mut super::get_all_input_arcs(3, 2));
    input_arcs.append(&mut super::get_all_input_arcs(3, 3));

    for input_arc in &input_arcs {
        let sn_out = sn_out_signal(&input_arc);
        txt += &format!("*---- arc : {:?} -> {:?} ----\n", input_arc, sn_out);

        let pre_vdd = format!("VDD_PRE_{}", input_arc.to_string());
        let target_vdd = format!("VDD_MAIN_{}", input_arc.to_string());
        txt += &line_source_dc(&pre_vdd, "avdd");
        txt += &line_source_dc(&target_vdd, "avdd");
        txt += &line_cell(&input_arc.to_string(), &[
            input_arc.0[0].to_wire(),
            input_arc.0[1].to_wire(),
            input_arc.0[2].to_wire(),
            "VBB",
            "VPP",
            "VSS",
            pre_vdd.as_str(),
            target_vdd.as_str(),
            "VDD",
            format!("d_A_{}", input_arc.to_string()).as_str(),
            format!("d_B_{}", input_arc.to_string()).as_str(),
            format!("d_CI_{}", input_arc.to_string()).as_str(),
            format!("z_CON_{}", input_arc.to_string()).as_str(),
            format!("z_SN_{}", input_arc.to_string()).as_str(),
            format!("q_CON_{}", input_arc.to_string()).as_str(),
            format!("q_SN_{}", input_arc.to_string()).as_str(),
        ], COVER);
        txt += &line_measure_power(&target_vdd);
    }

    for input_arc in &input_arcs {
        
        let pre_vdd = format!("VDD_PRE_{}", input_arc.to_string());
        
        txt += &line_measure_power(&pre_vdd);
    }

    txt
}


#[test]
fn test_dynamic() {
    let txt = dynamic();
    use std::fs::File;
    use std::io::prelude::*;
    let content = "This is the content to write to the file.";
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(txt.as_bytes());
}

#[test]
fn test_leakage() {
    let txt = leakage();
    use std::fs::File;
    use std::io::prelude::*;
    let content = "This is the content to write to the file.";
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(txt.as_bytes());
}

#[test]
fn test_timing() {
    let txt = timing_arc_measure_for_con_attend();
    use std::fs::File;
    use std::io::prelude::*;
    let content = "This is the content to write to the file.";
    // 创建一个新文件，如果文件已存在，则覆盖
    let mut file = File::create("output.txt").unwrap();
    // 将字符串写入文件
    let _ = file.write_all(txt.as_bytes());
}



#[test]
fn test() {
    for (type_arc, input_arc) in sorted_fa1n_input_arcs_with_type() {
        println!("{:?}", sn_out_signal(&input_arc));
    }
}

#[test]
fn test2() {
    for (type_arc, input_arc) in sorted_fa1n_input_arcs_with_type() {
        if con_out_signal(&input_arc).is_flip() {
            println!("{}\t{}\t{:?}", type_arc.to_str(), input_arc.to_string(), con_out_signal(&input_arc));
        }
    }
}
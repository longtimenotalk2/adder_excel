use crate::cell_parse::{CellSourceType, ProcessAndProject, RealCell};

pub mod cdl;

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
}
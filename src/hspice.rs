use crate::cell_parse::{CellSourceType, ProcessAndProject, RealCell};

pub mod cdl;

fn line_inc(path : &str) -> String {
    format!(".inc '{path}'\n")
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
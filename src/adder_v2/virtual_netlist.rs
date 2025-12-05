use std::collections::{BTreeMap, BTreeSet};

use crate::adder_v2::{Id, Port, adder::Adder, cell_parse::Process, excel::{ExcelFrame, excel_to_datalist::ExcelDataList}, wire::{Flag, Wire}};

#[derive(Debug, Clone)]
pub struct VirtualNetInst {
    pub inst_name : String,
    pub cell_logo : String,
    pub ports : BTreeMap<String, String>, // Port, Wire
}


impl Adder {
    pub fn to_virtual_netlist(&self, process : Process, cell_name_replace_map : BTreeMap<Id, String>) -> Vec<VirtualNetInst> {
        struct InstLine {
            inst_name : String,
            cell_name : String,
            port_and_wire : BTreeMap<Port, (Id, Wire)>
        }

        let mut wires: BTreeSet<(u32, Wire)> = BTreeSet::new();
        let mut inst_line: Vec<InstLine> = vec![];

        let layers = self.scan_layer_end_same();

        let same_wire_mapping = self.get_same_wire_mapping();

        let abs_wire_name = |input : &(Id, Wire)| -> String {
            let wire = &input.1;
            let mut base_name = wire.to_string_netlist();
            let sub_id = *same_wire_mapping.get(input).unwrap();
            if sub_id > 1 {
                // dbg!("!!");
                base_name = format!("{base_name}_slow_{sub_id}");
            }
            base_name
        };

        for (cell_id, cell) in self.cells.iter() {

            let layer = layers.get(*cell_id as usize).unwrap();

            let all_port_and_wire = cell.node.to_port_vs_wire();

            for (_, wire) in &all_port_and_wire {
                if wire.1.is_not_a_b_s() {
                    wires.insert(wire.clone());
                }
            }

            inst_line.push(InstLine {
                inst_name: cell.node.to_inst_name_with_layer(*layer),
                cell_name: {
                    if let Some(replaced_cell_name) = cell_name_replace_map.get(cell_id) {
                        replaced_cell_name.clone()
                    } else {
                        cell.to_cell_body().parse(process).0.0
                    }
                },
                port_and_wire: all_port_and_wire,
            })
        }

        let mut ret_list = vec![];

        for inst in inst_line.iter() {
            let mut ports = BTreeMap::new();
            for (i, (port, wire)) in inst.port_and_wire.iter().rev().enumerate() {
                let port_name = port.0.clone();
                let wire_name = abs_wire_name(wire);
                ports.insert(port_name, wire_name);
            }
            ret_list.push(VirtualNetInst {
                inst_name: inst.inst_name.clone(),
                cell_logo: inst.cell_name.clone(),
                ports,
            })
        }

        ret_list
    }
}

#[test]
fn test() {
    const PATH : &'static str = "src/adder_v2/project/a04_uf_76/excel/uf31_pn_np_v01.txt";
    fn adder()  -> Adder {
        adder_and_excel().0
    }

    fn adder_and_excel()  -> (Adder, ExcelDataList<Id>) {
        let excel_frame = ExcelFrame::load(PATH);
        let excel_data_list = ExcelDataList::from_excel_frame(&excel_frame);
        let (adder, excel_map) = Adder::create_from_excel_data_list(excel_data_list, false, true);
        adder.check_id_all_match();
        (adder, excel_map)
    }

    let adder = adder();
    let process = Process::N3E;

    dbg!(adder.to_virtual_netlist(process, BTreeMap::new()));
}
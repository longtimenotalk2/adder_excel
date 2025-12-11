use std::{collections::{BTreeMap, BTreeSet}, os::windows::process};

use crate::adder_v2::{adder::Adder, cell_parse::Process, draw::adder_frame::Pos, floorplan_m_v1::{CellId, CellPos, CellStaticData, FA1NInfo, FloorPlanMV1, WireId, WireStaticData}};

impl FloorPlanMV1 {
    pub fn init_from_adder(adder : &Adder, process: Process) -> Self {
        let virtual_netlist = adder.to_virtual_netlist(process, BTreeMap::new());
        let cell_width = adder.cells.iter().map(|x| x.1.to_cell_body().width()).collect::<Vec<_>>();

        let mut loaded_wire_names : BTreeMap<String, WireId> = BTreeMap::new();

        let mut wires : BTreeMap<WireId, WireStaticData> = BTreeMap::new();
        let mut cells : BTreeMap<CellId, CellStaticData> = BTreeMap::new();

        for id in 0..cell_width.len() {
            let width = cell_width[id];
            let cell_name = virtual_netlist[id].inst_name.clone();
            let cell_id = CellId(id as u16);
            let mut wire_id_set = BTreeSet::new();
            for (_, wire_name) in virtual_netlist[id].ports.iter() {
                if !loaded_wire_names.contains_key(wire_name) {
                    let wire_id = WireId(loaded_wire_names.len() as u16);
                    loaded_wire_names.insert(wire_name.clone(), wire_id);
                }
                let wire_id = loaded_wire_names[wire_name];
                wires.entry(wire_id).or_insert(WireStaticData { 
                    name: wire_name.clone(), 
                    connected_cell_set: BTreeSet::new(),
                }).connected_cell_set.insert(cell_id);
                wire_id_set.insert(wire_id);
            }
            cells.insert(cell_id, CellStaticData { 
                name: cell_name, 
                width: width as i32, 
                connected_wire_set: wire_id_set, 
            });
        }

        Self {
            cell_static_data : cells,
            wire_static_data : wires,
            cell_pos : BTreeMap::new(),
        }
    }

    pub fn load_adder_position(&mut self, path : &str) {
        let file = std::fs::File::open(path).expect(&format!("file {path} not exist"));
        let reader = std::io::BufReader::new(file);
        let lines : Vec<String> = std::io::BufRead::lines(reader).map(|l| l.unwrap()).collect();

        for line in lines {
            let tokens = line.split_whitespace().collect::<Vec<_>>();
            if tokens.len() > 0 {
                let name = tokens[0].to_string();
                let x_input : i32 = tokens[1].parse().unwrap();
                let y_input : i32 = tokens[2].parse().unwrap();

                let cell_id = self.find_cell_id_from_name(&name).unwrap();

                self.cell_pos.insert(cell_id, CellPos { x : x_input, y : y_input });
            }
        }
    }

    pub fn load_faa(&mut self, path : &str, fa1n_info : &BTreeMap<usize, FA1NInfo>) {
        let file = std::fs::File::open(path).expect(&format!("file {path} not exist"));
        let reader = std::io::BufReader::new(file);
        let lines : Vec<String> = std::io::BufRead::lines(reader).map(|l| l.unwrap()).collect();

        for line in lines {
            let tokens = line.split_whitespace().collect::<Vec<_>>();
            if tokens.len() > 0 {
                let name = tokens[0].to_string();
                if name.ends_with("s_0") || name.ends_with("co_0") {
                    // 双小cell

                    let x_input : i32 = tokens[1].parse().unwrap();
                    let y_input : i32 = tokens[2].parse().unwrap();

                    let cell_id = CellId(self.cell_static_data.len() as u16);
                    if name.ends_with("s_0") {
                        let wire_id = WireId(self.wire_static_data.len() as u16);
                        let width = 6;
                        self.cell_static_data.insert(cell_id, CellStaticData {
                            name: "s_0".to_string(),
                            width: width as i32,
                            connected_wire_set: BTreeSet::from([wire_id]),
                        });
                        self.cell_pos.insert(cell_id, CellPos { x: x_input, y: y_input });
                        self.wire_static_data.insert(wire_id, WireStaticData {
                            name: "d[0]".to_string(),
                            connected_cell_set: BTreeSet::from([cell_id]),
                        });
                    } else if name.ends_with("co_0") {
                        let wire_id = self.find_wire_id_from_name("b[0]").unwrap();
                        let width = 3;
                        let x = x_input as f64 + width as f64 / 2.0 ;
                        self.cell_static_data.insert(cell_id, CellStaticData {
                            name: "co_0".to_string(),
                            width: width as i32,
                            connected_wire_set: BTreeSet::from([wire_id]),
                        });
                        self.cell_pos.insert(cell_id, CellPos { x: x_input, y: y_input });
                    }
                } else {
                    // 大FA1N
                    let index : usize = tokens[0].split("_").last().unwrap().parse().unwrap();
                    let adder_index = index - 1;
                    let x : i32 = tokens[1].parse().unwrap();
                    let y : i32 = tokens[2].parse().unwrap();
                    let y = y + 1;
                    let width = fa1n_info.get(&index).unwrap().width();

                    let cell_id = CellId(self.cell_static_data.len() as u16);
                    let wire_a_id = self.find_wire_id_from_name(&format!("a[{adder_index}]")).unwrap();
                    let wire_b_id = self.find_wire_id_from_name(&format!("b[{index}]"));

                    let mut wires =  BTreeSet::from([wire_a_id]);
                    if let Some(wire_b_id) = wire_b_id {
                        wires.insert(wire_b_id);
                    }

                    let mut cell_name = format!("a[{adder_index}]");
                    if wire_b_id.is_some() {
                        cell_name += &format!("_b[{index}]");
                    }

                    self.cell_static_data.insert(cell_id, CellStaticData {
                        name: cell_name,
                        width,
                        connected_wire_set : wires,
                    });

                    self.cell_pos.insert(cell_id, CellPos{x, y});

                    self.wire_static_data.get_mut(&wire_a_id).unwrap().connected_cell_set.insert(cell_id);
                    if let Some(wire_b_id) = wire_b_id {
                        self.wire_static_data.get_mut(&wire_b_id).unwrap().connected_cell_set.insert(cell_id);
                    }
                }
                
            }
        }
    }

    pub fn load_mb32(&mut self, mb_data : &[(i32, i32); 32]) {
        for i in 0..32 {
            let (x, y) = mb_data[i];
            let wire_name = if i == 0 {
                "d[0]".to_string()
            } else {
                let bit = i - 1;
                format!("s[{}]", bit)
            };
            

            let cell_id = CellId(self.cell_static_data.len() as u16);
            let wire_id = self.find_wire_id_from_name(&wire_name).unwrap();

            self.cell_static_data.insert(cell_id, CellStaticData {
                name: wire_name.clone(),
                width: 1,
                connected_wire_set: BTreeSet::from([wire_id]),
            });

            self.cell_pos.insert(cell_id, CellPos { x, y });
            self.wire_static_data.get_mut(&wire_id).unwrap().connected_cell_set.insert(cell_id);

        }
    }
}
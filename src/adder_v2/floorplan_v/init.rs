use std::collections::{BTreeMap, BTreeSet};

use crate::{adder_v2::{adder::Adder, cell_parse::Process, floorplan_v::{AdderFPMain, CellId, CellStaticInfo, ModelParameters, Pos, SubArea, SubAreaId, WireId, WireStaticInfo}}, from_excel::load};

impl AdderFPMain {
    pub fn init_from_adder(
        adder: &Adder, 
        model : ModelParameters,
        process : Process,
    ) -> Self {
        let virtual_netlist = adder.to_virtual_netlist(process, BTreeMap::new());
        let cell_width = adder.cells.iter().map(|x| x.1.to_cell_body().width()).collect::<Vec<_>>();

        let mut main = Self::new(model);

        let mut loaded_wire_names : BTreeMap<String, WireId> = BTreeMap::new();

        let mut wires : BTreeMap<WireId, WireStaticInfo> = BTreeMap::new();
        let mut cells : BTreeMap<CellId, CellStaticInfo> = BTreeMap::new();

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
                wires.entry(wire_id).or_insert(WireStaticInfo { 
                    name: wire_name.clone(), 
                    connected_cells: BTreeSet::new(),
                }).connected_cells.insert(cell_id);
                wire_id_set.insert(wire_id);
            }
            cells.insert(cell_id, CellStaticInfo { 
                name: cell_name, 
                width, 
                can_move: true, 
                wires: wire_id_set, 
            });
        }

        main.cell_static_dict = cells;
        main.wire_static_dict = wires;

        main
    }

    pub fn load_subarea(&mut self, sub_area_list : Vec<SubArea>) {
        for (id, sub_area) in sub_area_list.into_iter().enumerate() {
            self.sub_area_dict.insert(SubAreaId(id as u16), sub_area);
        }
    }

    pub fn load_adder_cell_position(&mut self, path : &str) {
        let file = std::fs::File::open(path).expect(&format!("file {path} not exist"));
        let reader = std::io::BufReader::new(file);
        let lines : Vec<String> = std::io::BufRead::lines(reader).map(|l| l.unwrap()).collect();

        for line in lines {
            let tokens = line.split_whitespace().collect::<Vec<_>>();
            if tokens.len() > 0 {
                let name = tokens[0].to_string();
                let x_input : i32 = tokens[1].parse().unwrap();
                let y_input : i32 = tokens[2].parse().unwrap();

                let mut cell_id = None;
                let mut cell_info = None;
                for (id, cell) in self.cell_static_dict.iter() {
                    if cell.name == name {
                        cell_id = Some(*id);
                        cell_info = Some(cell);
                        break;
                    }
                }
                let cell_id = if let Some(cell_id) = cell_id {
                    cell_id
                } else {
                    panic!("cell {name} not found!")
                };
                let cell_info = cell_info.unwrap();

                let width = cell_info.width;

                let x = x_input as f64 + width as f64 / 2.0 ;

                let mut sub_area_id = self.get_sub_area_id(x, y_input);

                self.cell_pos_dict.insert(cell_id, Pos { x, sub_area_id });
            }
        }
    }

    pub fn load_faa_cell_position(&mut self, path : &str) {
        let file = std::fs::File::open(path).expect(&format!("file {path} not exist"));
        let reader = std::io::BufReader::new(file);
        let lines : Vec<String> = std::io::BufRead::lines(reader).map(|l| l.unwrap()).collect();

        for line in lines {
            let tokens = line.split_whitespace().collect::<Vec<_>>();
            if tokens.len() > 0 {
                let name = tokens[0].to_string();
                if name.ends_with("s_0") || name.ends_with("co_0") {
                    let x_input : i32 = tokens[1].parse().unwrap();
                    let y_input : i32 = tokens[2].parse().unwrap();

                    let cell_id = CellId(self.cell_static_dict.len() as u16);
                    if name.ends_with("s_0") {
                        let wire_id = WireId(self.wire_static_dict.len() as u16);
                        let width = 6;
                        let x = x_input as f64 + width as f64 / 2.0 ;
                        self.cell_static_dict.insert(cell_id, CellStaticInfo {
                            name: "s_0".to_string(),
                            width,
                            can_move : true,
                            wires: BTreeSet::from([wire_id]),
                        });
                        self.cell_pos_dict.insert(cell_id, Pos { x, sub_area_id : self.get_sub_area_id(x, y_input) });
                        self.wire_static_dict.insert(wire_id, WireStaticInfo {
                            name: "d[0]".to_string(),
                            connected_cells: BTreeSet::from([cell_id]),
                        });
                    } else if name.ends_with("co_0") {
                        let wire_id = self.get_wire_id_by_name("b[0]");
                        let width = 3;
                        let x = x_input as f64 + width as f64 / 2.0 ;
                        self.cell_static_dict.insert(cell_id, CellStaticInfo {
                            name: "co_0".to_string(),
                            width,
                            can_move : true,
                            wires: BTreeSet::from([wire_id]),
                        });
                        self.cell_pos_dict.insert(cell_id, Pos { x, sub_area_id : self.get_sub_area_id(x, y_input) });
                    }
                }
                
            }
        }
    }
}
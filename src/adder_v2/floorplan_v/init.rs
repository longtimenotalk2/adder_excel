use std::collections::{BTreeMap, BTreeSet};

use crate::{adder_v2::{adder::Adder, cell_parse::Process, floorplan_v::{AdderFPMain, CellId, CellStaticInfo, ModelParameters, SubArea, SubAreaId, WireId, WireStaticInfo}}, from_excel::load};

impl AdderFPMain {
    pub fn init_from_adder(
        adder: &Adder, 
        model : ModelParameters,
        sub_area_list: Vec<SubArea>,
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

        for (i, sub_area) in sub_area_list.into_iter().enumerate() {
            main.sub_area_dict.insert(SubAreaId(i as u16), sub_area);
        }

        main

    }
}
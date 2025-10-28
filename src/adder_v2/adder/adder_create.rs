use colorful::{Color, Colorful};

use crate::adder_v2::{adder::Adder, cell::{cell_info::CellInfo, Cell}, excel::excel_to_datalist::ExcelDataList, logic::Logic, node::{Node, NodeHint}, wire::{wire_list::WireList, Flag, Wire}, Id, Port};

impl Adder {
    pub fn create_from_excel_data_list(
        excel_data_list: ExcelDataList<(NodeHint, CellInfo, Option<Vec<i32>>)>,
        input_is_neg: bool,
        output_is_neg: bool,
    ) -> (Self, ExcelDataList<Id>) {
        let bits = excel_data_list.bits;
        // 首先假设输入是正，生成全进位链
        let mut wire_id : Id = 0;

        let mut wires: Vec<(Id, Wire)> = vec![];
        let mut cells: Vec<(Id, Cell)> = vec![];
        let mut excel_cell_map: ExcelDataList<Id> = ExcelDataList::new_with_bits(bits);

        for i in 0..bits {
            wires.push((wire_id, Wire::from_str(&format!("a{i}"))));
            wire_id += 1;
            wires.push((wire_id, Wire::from_str(&format!("b{i}"))));
            wire_id += 1;
        }

        let mut wire_list = WireList(wires);

        for (cell_id, (excel_key, (hint, cell_info, _))) in excel_data_list.data.iter().enumerate() {
            let cell_id = cell_id as Id;
            // dbg!(cell_id);
            match Node::create_from_hint(hint, &mut wire_list)  {
                Ok(node) => {
                    for output_wire in node.get_ordered_output_wires() {
                        wire_list.0.push(output_wire);
                    }
                    let cell = Cell::new(node, cell_info.clone());
                    cells.push((cell_id, cell));
                    excel_cell_map.data.insert(excel_key.clone(), cell_id);
                }
                Err(e) => {
                    println!("wire list : ");
                    for wire in &wire_list.0 {
                        println!("> {:02} : {}", wire.0, wire.1.to_string());
                    }
                    println!("node list : ");
                    for (id, cell) in cells.iter() {
                        println!("> {id:02} : {}", cell.node.to_string());
                    }
                    println!("{}", e.to_string());
                    panic!();
                }
            }
        }

        // -----增添尾部异或------

        let is_if_in_posi_than_end_is_neg = output_is_neg ^ input_is_neg;

        // 首比特处理
        let first_s_wire_should = Wire {
            flag : Flag::Q,
            is_neg : is_if_in_posi_than_end_is_neg,
            index : 0,
            len : 1,
        };
        let first_s_wire = Wire {
            flag : Flag::S,
            is_neg : is_if_in_posi_than_end_is_neg,
            index : 0,
            len : 1,
        };

        if wire_list.find(&first_s_wire_should).is_ok() {
            // 将该Wire替换为S
            
            wire_list.find_and_replace(&first_s_wire_should, first_s_wire.clone());
            for (_, cell) in cells.iter_mut() {
                let (_, z_out_wire) = &mut cell.node.io.output_z;
                if *z_out_wire == first_s_wire_should {
                    *z_out_wire = first_s_wire;
                    break;
                }
            }
        } else {
            // 找到与其相反的q并加一个INV
            let first_s_wire_should = Wire {
                flag : Flag::Q,
                is_neg : !is_if_in_posi_than_end_is_neg,
                index : 0,
                len : 1,
            };
            let found_wire = wire_list.find(&first_s_wire_should).unwrap();
            let new_wire_id = wire_list.0.len() as Id;
            wire_list.0.push((new_wire_id, first_s_wire.clone()));

            let inv_cell_id = cells.len() as Id;
            let inv_cell = Cell::new(
                Node::create_by_ordered_wires(Logic::INV, vec![found_wire, (new_wire_id, first_s_wire)]),
                CellInfo::default(),
            );
            cells.push((inv_cell_id, inv_cell));
        }

        // 后续比特处理，默认抓取ID较小者
        for index in 1..bits {
            let s_wire = Wire {
                flag : Flag::S,
                is_neg : is_if_in_posi_than_end_is_neg,
                index,
                len : 1,
            };
            let new_wire_id = wire_list.0.len() as Id;
            wire_list.0.push((new_wire_id, s_wire.clone()));
            let new_cell_id = cells.len() as Id;
            // find g index-1 to 0
            let mut g_list : Vec<(Id, Wire)> = vec![];
            for is_neg in [true, false] {
                if let Ok(ret) = wire_list.find(&Wire::new(Flag::G, is_neg, index-1, index)) {
                    g_list.push(ret);
                }
            }
            g_list.sort_by(|a, b| a.0.cmp(&b.0));
            if g_list.len() > 1 {
                let mut txt = format!(">>> {} : for node s{index}, multi c can be select : ", "warning".color(Color::Orange1));
                txt += &format!("{} ", g_list[0].1.to_string().color(Color::Green));
                for g in &g_list[1..] {
                    txt += &format!("{} ", g.1.to_string());
                }
                println!("{}", txt);
            }
            let g = g_list[0].clone();
            // find q index 
            let mut q_list : Vec<(Id, Wire)> = vec![];
            for is_neg in [true, false] {
                if let Ok(ret) = wire_list.find(&Wire::new(Flag::Q, is_neg, index, 1)) {
                    q_list.push(ret);
                }
            }
            q_list.sort_by(|a, b| a.0.cmp(&b.0));
            if q_list.len() > 1 {
                let mut txt = format!(">>> {} : for node s{index}, multi c can be select : ", "warning".color(Color::Orange1));
                txt += &format!("{} ", q_list[0].1.to_string().color(Color::Green));
                for q in &q_list[1..] {
                    txt += &format!("{} ", q.1.to_string());
                }
                println!("{}", txt);
            }
            let q = q_list[0].clone();
            let logic = if g.1.is_neg ^ q.1.is_neg ^ is_if_in_posi_than_end_is_neg {
                Logic::XNR2
            } else {
                Logic::XOR2
            };
            cells.push((new_cell_id, Cell::new(Node::create_by_ordered_wires(logic, vec![
                g,
                q,
                (new_wire_id, s_wire.clone()),
            ]), CellInfo::default())));
        }

        // -----处理输入就是反的情况------

        // mirror所有cell
        if input_is_neg {
            for (_, cell) in cells.iter_mut() {
                cell.node.logic = cell.node.logic.mirror();
            }
        }

        (
            Adder {
                bits,
                wires : wire_list.0,
                cells,
                input_is_neg,
                output_is_neg,
            },
            excel_cell_map,
        )
    }
}
use std::{cell, collections::BTreeMap};

use crate::adder_v2::{Id, adder::Adder, cell_parse::Process, excel::{ExcelFrame, excel_to_datalist::ExcelDataList}, floorplan_v::{AdderFPMain, ModelParameters, SubArea, SuperParameters}};

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

fn get_sub_area_v1() -> Vec<SubArea> {
    let mut sub_area_list = vec![];

    let x_len = 192.;

    for y in [2, 3] {
        sub_area_list.push(SubArea {
            y,
            x_min : 9.,
            x_max : 43.,
        });
        sub_area_list.push(SubArea {
            y,
            x_min : 60.,
            x_max : 94.,
        });
        sub_area_list.push(SubArea {
            y,
            x_min : 111.,
            x_max : 156.,
        });
        sub_area_list.push(SubArea {
            y,
            x_min : 170.,
            x_max : 192.,
        });
    }

    for y in [4,5,6] {
        sub_area_list.push(SubArea {
            y,
            x_min : 0.,
            x_max : x_len,
        })
    }

    sub_area_list

}

fn fa1n_width_dict() -> BTreeMap<usize, f64> {
    let mut dict = BTreeMap::new();
    for i in 1..32 {
        if  i <= 17 {
            dict.insert(i, 16./2.);
        } else {
            dict.insert(i, 14./2.);
        }
    }
    dict
}

fn init() -> (AdderFPMain, Adder) {
    let adder = adder();
    let process = Process::N3E;
    let mut fp_main = AdderFPMain::init_from_adder(&adder, ModelParameters::for_76(), process);

    let sub_area_list = get_sub_area_v1();
    
    fp_main.load_subarea(sub_area_list);

    fp_main.load_adder_cell_position("src/adder_v2/floorplan_v/project/a01_ufv01/input_adder_placement.txt");
    fp_main.load_faa_cell_position("src/adder_v2/floorplan_v/project/a01_ufv01/input_faa_placement.txt");
    fp_main.set_out_mb_virtual_cell(7, 192., 31);
    fp_main.set_in_fa1n_virtual_cell("src/adder_v2/floorplan_v/project/a01_ufv01/input_faa_placement.txt", &fa1n_width_dict());

    (fp_main, adder)
}


#[test]
fn test_draw() {
    let (fp_main, adder) = init();
    fp_main.draw_default_art(&adder, "place_init");
}

#[test]
fn test_dynamic() {
    let (mut fp_main, adder) = init();
    let super_parameters = SuperParameters {
        alpha_wire_energy : 1.,
        alpha_density_energy : 1.,
        alpha_border_energy : 1e4,
        alpha_overlap_energy : 1e1,
    };
    let mut beta = 2.;
    // let cell_id = fp_main.get_cell_id_by_name("U1_nh3_2");
    // dbg!(&cell_id);
    // dbg!(fp_main.given_cell_x_energy(cell_id, &super_parameters));
    fp_main.dynamic_main_x(beta, &super_parameters);
    // dbg!(fp_main.given_cell_x_energy(cell_id, &super_parameters));
    fp_main.draw_default_art(&adder, "place_dynamic1");
    fp_main.dynamic_main_x(beta, &super_parameters);
    fp_main.draw_default_art(&adder, "place_dynamic2");
    
    for i in 0..8 {
        beta *= 0.95;
        fp_main.dynamic_main_x(beta, &super_parameters);
    }

    fp_main.draw_default_art(&adder, "place_dynamic10");

    fp_main.dynamic_main_y(&super_parameters);
    fp_main.draw_default_art(&adder, "place_dynamic11");

    for i in 0..9 {
        beta *= 0.95;
        fp_main.dynamic_main_x(beta, &super_parameters);
    }
    fp_main.draw_default_art(&adder, "place_dynamic20");

    // let super_parameters = SuperParameters {
    //     alpha_wire_energy : 1.,
    //     alpha_density_energy : 1.,
    //     alpha_border_energy : 1e8,
    //     alpha_overlap_energy : 1e4,
    // };

    // for i in 0..30 {
    //     beta *= 0.95;
    //     fp_main.dynamic_main_x(beta, &super_parameters);
    // }

    // fp_main.draw_default_art(&adder, "place_dynamic40");


}

#[test]
fn test_dynamic2() {
    let (mut fp_main, adder) = init();
    let super_parameters = SuperParameters {
        alpha_wire_energy : 1.,
        alpha_density_energy : 1.,
        alpha_border_energy : 1e4,
        alpha_overlap_energy : 1e1,
    };
    let mut beta = 2.;


    fp_main.dynamic_combine_5_step(beta, &super_parameters);
    fp_main.draw_default_art(&adder, "place_dynamic5");

    for i in 0..3 {
        beta *= 0.9;
        fp_main.dynamic_combine_5_step(beta, &super_parameters);
    }

    fp_main.draw_default_art(&adder, "place_dynamic20");

    let super_parameters = SuperParameters {
        alpha_wire_energy : 1.,
        alpha_density_energy : 1.,
        alpha_border_energy : 1e4,
        alpha_overlap_energy : 1e2,
    };

    for i in 0..6 {
        beta *= 0.9;
        fp_main.dynamic_combine_5_step(beta, &super_parameters);
    } 

    fp_main.draw_default_art(&adder, "place_dynamic50");

    let super_parameters = SuperParameters {
        alpha_wire_energy : 1.,
        alpha_density_energy : 1.,
        alpha_border_energy : 1e4,
        alpha_overlap_energy : 1e3,
    };

    for i in 0..6 {
        beta *= 0.95;
        fp_main.dynamic_combine_5_step(beta, &super_parameters);
    } 

    fp_main.draw_default_art(&adder, "place_dynamic80");

    let super_parameters = SuperParameters {
        alpha_wire_energy : 1.,
        alpha_density_energy : 1.,
        alpha_border_energy : 1e5,
        alpha_overlap_energy : 1e4,
    };

    for i in 0..6 {
        beta *= 0.95;
        fp_main.dynamic_combine_5_step(beta, &super_parameters);
    } 

    fp_main.draw_default_art(&adder, "place_dynamic110");

}

#[test]
fn test_debug() {
    let (fp_main, adder) = init();

    let cell_id = fp_main.get_cell_id_by_name("U1_np1_q1");
    dbg!(fp_main.given_cell_y_wire_force_atom(cell_id));
}




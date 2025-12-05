use crate::adder_v2::{Id, adder::Adder, cell_parse::Process, excel::{ExcelFrame, excel_to_datalist::ExcelDataList}, floorplan_v::{AdderFPMain, ModelParameters, SubArea}};

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


#[test]
fn test() {
    let adder = adder();
    let process = Process::N3E;
    let mut fp_main = AdderFPMain::init_from_adder(&adder, ModelParameters::for_76(), process);

    let sub_area_list = get_sub_area_v1();
    
    fp_main.load_subarea(sub_area_list);

    fp_main.load_adder_cell_position("src/adder_v2/floorplan_v/project/a01_ufv01/input_adder_placement.txt");

    // dbg!(fp_main);
    fp_main.draw_default_art(&adder);
}




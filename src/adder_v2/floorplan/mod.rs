use crate::adder_v2::{Id, adder::Adder, excel::{ExcelFrame, excel_to_datalist::ExcelDataList}};

pub mod cell_area;
mod auto_place;

#[test]
fn test1_bkf() {
    const PATH : &'static str = "src/adder_v2/project/a05_bkf_76/excel/bkf31_pp_nn_v02.txt";

    fn adder()  -> Adder {
        adder_and_excel().0
    }

    fn adder_and_excel()  -> (Adder, ExcelDataList<Id>) {
        let excel_frame = ExcelFrame::load(PATH);
        let excel_data_list = ExcelDataList::from_excel_frame(&excel_frame);
        let (adder, excel_map) = Adder::create_from_excel_data_list(excel_data_list, false, false);
        adder.check_id_all_match();
        (adder, excel_map)
    }

    let place = adder().auto_place();

    place.draw();
}

#[test]
fn test2_uf() {
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

    let place = adder().auto_place();

    place.draw();
}
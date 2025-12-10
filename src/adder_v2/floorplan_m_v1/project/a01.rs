use crate::adder_v2::{Id, adder::Adder, cell_parse::Process, excel::{ExcelFrame, excel_to_datalist::ExcelDataList}, floorplan_m_v1::FloorPlanMV1};

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


#[test]
fn test_base() {
    let adder = adder();
    let mut fp = FloorPlanMV1::init_from_adder(&adder, Process::N3E);
    fp.load_adder_position("src/adder_v2/floorplan_m_v1/project/a01_data/a01_input_adder.txt");

    dbg!(fp);
}
use crate::adder_v2::{adder::Adder, excel::{excel_to_datalist::ExcelDataList, ExcelFrame}, wire::Wire, Id};

const PATH : &'static str = "src/adder_v2/project/a00_test/excel/test00.txt";

fn adder()  -> Adder {
    adder_and_excel().0
}

fn adder_and_excel()  -> (Adder, ExcelDataList<Id>) {
    let excel_frame = ExcelFrame::load(PATH);
    let excel_data_list = ExcelDataList::from_excel_frame(&excel_frame);
    let (adder, excel_map) = Adder::create_from_excel_data_list(excel_data_list, true, false);
    adder.check_id_all_match();
    (adder, excel_map)
}

#[test]
fn test_excel_frame() {
    let excel_frame = ExcelFrame::load(PATH);
    // dbg!(excel_frame);
    let excel_data_list = ExcelDataList::from_excel_frame(&excel_frame);
    dbg!(excel_data_list);
}

#[test]
fn test_adder() {
    let adder = adder();
    // println!("{}", adder.to_string());
}

#[test]
fn test_adder_function() {
    let adder = adder();
    adder.check_function_random(100);
}

#[test]
fn test_adder_cap() {
    let excel_frame = ExcelFrame::load(PATH);
    let (adder, excel_map) = adder_and_excel();
    adder.get_all_cap_by_excel(&excel_frame, &excel_map);
}

// #[test]
// fn test_adder_cap_debug() {
//     let adder = adder();
//     let wire = &adder.cells.get(115).unwrap().1.node.io.output_z;
//     dbg!(&wire);
//     let cap = adder.get_cap_cmos_for_wire(&wire);
//     dbg!(cap);
// }

#[test]
fn test_adder_property() {
    let adder = adder();
    let set = adder.cell_body_set();
    for cell in set {
        println!("{}", cell.to_string())
    }
}
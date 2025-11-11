use crate::hspice::other::cell::InputArc;

fn fa1n_input_arcs() -> Vec<InputArc> {
    super::get_all_input_arcs(3, 1)
}

#[test]
fn test() {
    dbg!(fa1n_input_arcs());
}
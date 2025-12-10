pub fn out_x_v1d8d11(y_mb : i32) -> [(i32, i32); 32] {
    let mut out_x : [(i32, i32); 32] = [(0, 0); 32];
    for index in 0..32 {
        if index <= 16 {
            if index % 2 == 0 {
                out_x[index] = ((5 + index / 2 * 11) as i32, y_mb);
            } else {
                out_x[index] = ((6 + (index-1) / 2 * 11) as i32, y_mb);
            }
        } else if index == 17 {
            out_x[index] = (100, y_mb - 1);
        } else if index == 18 {
            out_x[index] = (110, y_mb - 1);
        } else if index == 19 {
            out_x[index] = (111, y_mb - 1);
        } else {
            let index_new = index - 20;
            if index % 2 == 0 {
                out_x[index] = ((110 + index_new / 2 * 11) as i32, y_mb);
            } else {
                out_x[index] = ((111 + (index_new-1) / 2 * 11) as i32, y_mb);
            }
        }
    }

    out_x
}

#[test]
fn test_out_x_v1d8d11() {
    dbg!(out_x_v1d8d11(8));
}
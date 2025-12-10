use std::collections::BTreeSet;

use crate::adder_v2::floorplan_m_v1::{FA1NCate, FA1NInfo, FA1NPose, M1Certain, M1XEnum, M1YRange};

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

impl FA1NInfo {
    // fn width(&self) -> i32 {
    //     match (self.cate, self.pose) {
    //         (FA1NCate::CONBUF, FA1NPose::Left) => 9,
    //         (FA1NCate::CONBUF, FA1NPose::Right) => 8,
    //         (FA1NCate::W, FA1NPose::Left) => 7,
    //         (FA1NCate::W, FA1NPose::Right) => 7,
    //     }
    // }

    fn inner_m1(&self) -> BTreeSet<M1Certain> {
        match (self.cate, self.pose) {
            (FA1NCate::CONBUF, FA1NPose::Left) => BTreeSet::from([M1Certain {x : 5, y : M1YRange::new_range(0, 1)}]),
            (FA1NCate::CONBUF, FA1NPose::Right) => BTreeSet::from([M1Certain {x : 2, y : M1YRange::new_range(0, 1)}]),
            (FA1NCate::W, FA1NPose::Left) => BTreeSet::from([M1Certain {x : 6, y : M1YRange::new_single(0)}]),
            (FA1NCate::W, FA1NPose::Right) => BTreeSet::from([M1Certain {x : 0, y : M1YRange::new_single(1)}]),
        }
    }

    fn port_sn(&self) -> BTreeSet<(M1XEnum, M1YRange)> {
        match (self.cate, self.pose) {
            (FA1NCate::CONBUF, FA1NPose::Left) => BTreeSet::from([(M1XEnum::new(&[4, 6]), M1YRange::new_single(0))]),
            (FA1NCate::CONBUF, FA1NPose::Right) => BTreeSet::from([(M1XEnum::new(&[1, 3]), M1YRange::new_single(0))]),
            (FA1NCate::W, FA1NPose::Left) => BTreeSet::from([(M1XEnum::new(&[3, 4, 5]), M1YRange::new_single(0))]),
            (FA1NCate::W, FA1NPose::Right) => BTreeSet::from([(M1XEnum::new(&[1, 2, 3]), M1YRange::new_single(0))]),
        }
    }

    fn port_con(&self) -> BTreeSet<(M1XEnum, M1YRange)> {
        match (self.cate, self.pose) {
            (FA1NCate::CONBUF, FA1NPose::Left) => BTreeSet::from([(M1XEnum::new(&[6, 7, 8]), M1YRange::new_single(1))]),
            (FA1NCate::CONBUF, FA1NPose::Right) => BTreeSet::from([(M1XEnum::new(&[0, 1]), M1YRange::new_single(1))]),
            (FA1NCate::W, FA1NPose::Left) => BTreeSet::from([(M1XEnum::new(&[3, 4]), M1YRange::new_range(0, 1))]),
            (FA1NCate::W, FA1NPose::Right) => BTreeSet::from([(M1XEnum::new(&[2, 3]), M1YRange::new_range(0, 1))]),
        }
    }
}

#[test]
fn test_out_x_v1d8d11() {
    dbg!(out_x_v1d8d11(8));
}
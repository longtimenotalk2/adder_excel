use crate::adder_v2::logic::Logic;

impl Logic {
    pub fn color_hex_inner(&self) -> &'static str {
        match self {
            Logic::INV => "#DDEBF7",
            Logic::ND2 | Logic::NR2 | Logic::AN2 | Logic::OR2 | Logic::IND2 | Logic::INR2 => "#F8CBAD",
            Logic::AOI21 | Logic::OAI21 | Logic::AO21 | Logic::OA21 | Logic::IOAI21 | Logic::IAOI21 => "#D9D9D9",
            Logic::XNR2 | Logic::XNR2DOUT => "#FFC000",
            Logic::XOR2 | Logic::XOR2DOUT => "#FF99CC",
            Logic::AOI22 => "#A9D08E",
            Logic::OAI22 => "#E2EFDA",
            Logic::AOAI211 | Logic::OAOI211 | Logic::AOA211 | Logic::OAO211 => "#00FFFF",
            Logic::ND3 | Logic::NR3 => "#F4B084",
            Logic::AOAOI2111 | Logic::OAOAI2111 => "#BFBFBF",
            Logic::SUM => "#BDD7EE",
        }
    }

    pub fn string_for_show(&self) -> Vec<String> {
        match self {
            Logic::XNR2DOUT => vec!["XNR2".to_string(), "DOUT".to_string()],
            Logic::XOR2DOUT => vec!["XOR2".to_string(), "DOUT".to_string()],
            Logic::AOAOI2111 => vec!["AOAOI".to_string(), "2111".to_string()],
            Logic::OAOAI2111 => vec!["OAOAI".to_string(), "2111".to_string()],
            _ => vec![format!("{:?}", self)]
        }
    }
}


#[test]
fn test_rgb() {
    let r = 189;
    let g = 215;
    let b = 238;
    let hex = format!("{:02X}{:02X}{:02X}", r, g, b);
    println!("{}", hex);
}
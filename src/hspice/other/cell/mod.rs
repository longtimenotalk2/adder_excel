use std::fmt::Debug;

pub mod fa1n;
pub mod maj;

const LOW : &'static str = "i_LOW";
const HIGH : &'static str = "i_HIGH";
const FALL : &'static str = "i_FALL";
const RISE : &'static str = "i_RISE";

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Signal {
    Zero,
    One,
    Fall,
    Rise,
}

impl Signal {
    fn is_flip(&self) -> bool {
        match self {
            Signal::Fall | Signal::Rise => true,
            _ => false,
        }
    }

    fn start(&self) -> bool {
        match self {
            Signal::Zero | Signal::Rise => false,
            Signal::One | Signal::Fall => true,
        }
    }

    fn end(&self) -> bool {
        match self {
            Signal::Zero | Signal::Fall => false,
            Signal::One | Signal::Rise => true,
        }
    }

    fn from_start_and_end(start: bool, end: bool) -> Self {
        match (start, end) {
            (false, false) => Signal::Zero,
            (true, true) => Signal::One,
            (false, true) => Signal::Rise,
            (true, false) => Signal::Fall,
        }
    }

    fn to_wire(&self) -> &'static str {
        match self {
            Signal::Zero => LOW,
            Signal::One => HIGH,
            Signal::Fall => FALL,
            Signal::Rise => RISE,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Signal::Zero => "0".to_string(),
            Signal::One => "1".to_string(),
            Signal::Fall => "f".to_string(),
            Signal::Rise => "r".to_string(),
        }
    }
}

impl Debug for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Signal::Zero => write!(f, "0"),
            Signal::One => write!(f, "1"),
            Signal::Fall => write!(f, "f"),
            Signal::Rise => write!(f, "r"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct InputArc(Vec<Signal>);

impl InputArc {
    fn start(&self) -> Vec<bool> {
        self.0.iter().map(|s| s.start()).collect()
    }

    fn end(&self) -> Vec<bool> {
        self.0.iter().map(|s| s.end()).collect()
    }

    fn to_string(&self) -> String {
        self.0.iter().map(|s| format!("{}", s.to_string())).collect::<String>()
    }
}

impl Debug for InputArc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in self.0.iter() {
            write!(f, "{:?}", s)?;
        }
        Ok(())
    }
}

fn get_all_input_arcs(input_pins_num : usize, flip_num : usize) -> Vec<InputArc> {
    let mut input_arcs: Vec<InputArc> = vec![];
    let n  = 2_i32.pow(2 * input_pins_num as u32);
    
    for code in 0..n {
        let mut input_arc = vec![];
        for i in 0..input_pins_num {
            let bit4 = (code >> (2 * i as u32)) & 0b11;
            input_arc.push(match bit4 {
                0 => Signal::Zero,
                1 => Signal::One,
                2 => Signal::Fall,
                3 => Signal::Rise,
                _ => unimplemented!()
            });
        }
        let mut rf_count = 0;
        for c in input_arc.iter() {
            if c.is_flip() {
                rf_count += 1;
            }
        }
        if rf_count == flip_num {
            input_arcs.push(InputArc(input_arc));
        }
    }
    input_arcs
}
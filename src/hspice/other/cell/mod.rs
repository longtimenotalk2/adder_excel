use std::fmt::Debug;

pub mod fa1n;

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

#[derive(Clone)]
struct InputArc(Vec<Signal>);

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
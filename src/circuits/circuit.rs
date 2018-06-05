use super::{Current, Resistance, Voltage};

#[derive(Debug)]
pub enum Circuit {
    Resistor(Resistance, Voltage, Current),
    Parallel(Box<[Circuit]>, Voltage, Current),
    Series(Box<[Circuit]>, Voltage, Current),
}

impl Circuit {
    pub fn print(&self) -> String {
        let mut s = String::new();
        let _ = self.print_helper(&mut s, 1);
        s
    }

    fn print_helper(&self, s: &mut String, n: u32) -> u32 {
        let mut current_offset = n;
        match self {
            &Circuit::Resistor(Resistance(r), Voltage(v), Current(c)) => {
                s.push_str(&format!(
                    "r{}.\tR: {:.3} Ohms,\tV: {:.3} Volts,\tI: {:.3} Amps\n",
                    n, r, v, c
                ));
                n + 1
            }
            &Circuit::Series(ref cir, _, _) => {
                cir.iter()
                    .for_each(|c| current_offset = c.print_helper(s, current_offset));
                current_offset
            }
            &Circuit::Parallel(ref cir, _, _) => {
                cir.iter()
                    .for_each(|c| current_offset = c.print_helper(s, current_offset));
                current_offset
            }
        }
    }
}

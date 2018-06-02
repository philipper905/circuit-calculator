use super::Resistance;
use super::rayon::prelude::*;

use super::Circuit;
use quantities::{ohms_law, Either};

#[derive(Debug, PartialEq)]
pub enum Path {
    Resistor(Resistance),
    Parallel(Box<[Path]>),
    Series(Box<[Path]>),
}

impl Path {
    pub fn total_resistance(&self) -> Resistance {
        let one: f64 = 1.;
        let zero: f64 = 0.;

        match *self {
            Path::Resistor(r) => r,
            Path::Parallel(ref p) => {
                Resistance(one)
                    / (*p).par_iter()
                        .map(|r| Resistance(one) / r.total_resistance())
                        .reduce(|| Resistance(zero), |a, b| a + b)
            }
            Path::Series(ref s) => (*s).par_iter()
                .map(|r| r.total_resistance())
                .reduce(|| Resistance(zero), |a, b| a + b),
        }

        //        match *self {
        //            Path::Resistor(r) => r,
        //            Path::Parallel(ref p) => {
        //                Resistance(one) / p.iter().fold(Resistance(zero), |acc, current_path| {
        //                    acc + (Resistance(one) / current_path.total_resistance())
        //                })
        //            }
        //            Path::Series(ref p) => p.iter().fold(Resistance(zero), |acc, current_path| {
        //                acc + current_path.total_resistance()
        //            }),
        //        }
    }

    pub fn to_circuit(&self, either: &Either) -> Circuit {
        let total_resistance = self.total_resistance();
        let (total_voltage, total_current) = ohms_law(total_resistance, either);

        match *self {
            Path::Resistor(r) => Circuit::Resistor(r, total_voltage, total_current),
            Path::Series(ref s) => Circuit::Series(
                s.par_iter()
                    .map(|p_sub| p_sub.to_circuit(&Either::C(total_current)))
                    .collect::<Vec<Circuit>>()
                    .into_boxed_slice(),
                total_voltage,
                total_current,
            ),
            Path::Parallel(ref p) => Circuit::Parallel(
                p.par_iter()
                    .map(|p_sub| p_sub.to_circuit(&Either::V(total_voltage)))
                    .collect::<Vec<Circuit>>()
                    .into_boxed_slice(),
                total_voltage,
                total_current,
            ),
        }
    }
}

#[test]
fn add_series() {
    assert_eq!(
        Path::Series(vec![
            Path::Resistor(Resistance(1.0)),
            Path::Resistor(Resistance(2.0)),
            Path::Resistor(Resistance(3.0))
        ].into_boxed_slice()).total_resistance(),

        Resistance(6.0)
    );
}

#[test]
fn add_parallel() {
    assert_eq!(
        Path::Parallel(vec![
            Path::Resistor(Resistance(3.0)),
            Path::Resistor(Resistance(3.0)),
            Path::Resistor(Resistance(3.0))
        ].into_boxed_slice()).total_resistance(),

        Resistance(1.0)
    );
}

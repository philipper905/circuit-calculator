use std::ops;

use super::current::Current;
use super::resistance::Resistance;

#[derive(Debug, Copy, Clone)]
pub struct Voltage(pub f64);

impl ops::Div<Resistance> for Voltage {
    type Output = Current;

    fn div(self, rhs: Resistance) -> Self::Output {
        let (Voltage(v), Resistance(r)) = (self, rhs);
        Current(v / r)
    }
}

use std::ops;

use super::resistance::Resistance;
use super::voltage::Voltage;

#[derive(Debug, Copy, Clone)]
pub struct Current(pub f64);

impl ops::Mul<Resistance> for Current {
    type Output = Voltage;

    fn mul(self, rhs: Resistance) -> Self::Output {
        rhs.mul(self)
    }
}

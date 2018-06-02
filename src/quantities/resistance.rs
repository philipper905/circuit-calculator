use super::current::Current;
use super::voltage::Voltage;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Resistance(pub f64);

impl ops::Add for Resistance {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (Resistance(l), Resistance(r)) = (self, rhs);
        Resistance(l + r)
    }
}

impl ops::Div for Resistance {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let (Resistance(l), Resistance(r)) = (self, rhs);
        (Resistance(l / r))
    }
}

impl ops::Mul<Current> for Resistance {
    type Output = Voltage;

    fn mul(self, rhs: Current) -> Self::Output {
        let (Resistance(r), Current(c)) = (self, rhs);
        Voltage(r * c)
    }
}

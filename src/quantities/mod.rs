mod current;
mod resistance;
mod voltage;

pub use self::current::Current;
pub use self::resistance::Resistance;
pub use self::voltage::Voltage;

pub enum Either {
    V(Voltage),
    C(Current),
}

pub fn ohms_law(resistance: Resistance, other: &Either) -> (Voltage, Current) {
    match *other {
        Either::V(voltage) => (voltage, voltage / resistance),
        Either::C(current) => (current * resistance, current),
    }
}

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnclosedParallel,
    UnclosedSeries,
    UnclosedUnknown,
    UnopenedCircuit,
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::UnclosedParallel => "Forgot to close parallel circuit, expected an '>' and got an ']'",
            Error::UnclosedSeries   => "Forgot to close series circuit, expected an ']' and got an '>'",
            Error::UnclosedUnknown  => "Got to the end of parsing, and there was still a circuit unclosed, meaning either a ']' or a '>' was missing",
            Error::UnopenedCircuit  => "Seems like the circuit was never opened"
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = "Unclosed Series";
        let b = "Unclosed Parallel";
        let c = "Unclosed Unknown";
        let d = "Unopened Circuit";

        write!(
            f,
            "{}",
            match self {
                Error::UnclosedSeries => a,
                Error::UnclosedParallel => b,
                Error::UnclosedUnknown => c,
                Error::UnopenedCircuit => d,
            }
        )
    }
}

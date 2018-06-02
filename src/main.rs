// Todo:
// Change parser to enable typing in arbitrary floating point numbers
// good plan is to use the decimal crate --will check if it can convert back to float --ughh it seems like it can't
// I think I'll be able to do without though

// next would be to refactor the parse so it's not the 70 line clusterfuck that it is right now.

// Handy script to update the symlink:
// ln -sf /Users/Philippe/decuments/programs/rust/circuit/target/release/circuit /usr/local/bin

extern crate rayon;

mod quantities;
use quantities::{Either, Voltage};

mod circuits;
use circuits::EditablePath;

mod error;

use std::error::Error as StdError;
use std::io;
use std::io::Write;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    let mut buffer = String::new();

    let editable_path = get_editable_path(&mut out, &stdin, &mut buffer);
    let path = editable_path.into_immutable_path();

    let resistance: f64 = path.total_resistance().0;
    writeln!(&mut out, "\nTotal Resistance: {:?} Ohms\n", resistance).unwrap();

    let voltage = get_voltage(&mut out, &stdin, &mut buffer);

    let c = path.to_circuit(&Either::V(voltage));

    //    println!("{:#?}", c);
    writeln!(&mut out, "\n{}", c.print()).unwrap();
    //    println!("{}", print(&c));
}

fn get_editable_path(
    stdout_lock: &mut io::StdoutLock,
    stdin: &io::Stdin,
    buffer: &mut String,
) -> EditablePath {
    let editable_path: EditablePath;

    loop {
        writeln!(
            stdout_lock,
            "Please input your circuit (<> for parallel circuits, [] for series circuits)"
        ).unwrap();
        let _ = stdin.read_line(buffer);
        match EditablePath::parse(buffer) {
            Ok(e_path) => {
                editable_path = e_path;
                buffer.clear();
                break;
            }
            Err(e) => {
                writeln!(stdout_lock, "{}\n", e.description()).unwrap();

                buffer.clear();
            }
        }
    }

    editable_path
}

fn get_voltage(
    stdout_lock: &mut io::StdoutLock,
    stdin: &io::Stdin,
    buffer: &mut String,
) -> Voltage {
    let voltage: Voltage;
    loop {
        let _ = writeln!(stdout_lock, "What is the total voltage of your system?");
        let _ = stdin.read_line(buffer);
        match buffer.trim().parse() {
            Ok(num) => {
                voltage = Voltage(num);
                break;
            }
            Err(_) => {
                let _ = writeln!(stdout_lock, "Please enter a valid number");
                buffer.clear();
            }
        }
    }

    voltage
}

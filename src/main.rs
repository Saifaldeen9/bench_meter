// src/main.rs
// This file is the CRATE ROOT of our binary crate.
// The compiler starts here and builds the module tree from the `mod` lines.

mod circuits; // look for src/circuits.rs (or src/circuits/mod.rs)
mod report; // look for src/report.rs
mod units; // look for src/units.rs

// `use` creates short names (shortcuts) inside this module.
use circuits::{parallel_total, series_total}; // re-exported by circuits.rs
use units::{Ohms, Volts};

fn main() {
    // Three resistors from the parts drawer.
    let resistors = [Ohms::new(100.0), Ohms::new(220.0), Ohms::new(330.0)];

    let series = series_total(&resistors);
    let parallel = parallel_total(&resistors);

    // A 5 V supply connected to the series chain.
    let supply = Volts(5.0); // OK: the field of Volts is public
    let current = units::current(supply, series);

    println!("=== Bench Meter ===");
    println!("{}", report::resistance_line("Series", series));
    println!("{}", report::resistance_line("Parallel", parallel));
    println!("{}", report::current_line(supply, series, current));
}

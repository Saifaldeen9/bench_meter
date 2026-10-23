// src/circuits.rs  ->  module `crate::circuits`
// This file declares two child modules that live in the folder src/circuits/.

pub mod parallel; // src/circuits/parallel.rs
pub mod series; // src/circuits/series.rs

// Re-exports: callers can write `circuits::series_total`
// instead of `circuits::series::total`.
pub use parallel::total as parallel_total;
pub use series::total as series_total;

use crate::units::Ohms;

// A PRIVATE helper. It is not `pub`, but child modules (series, parallel)
// can still call it with `super::check_not_empty`, because
// a child module can see the private items of its ancestors.
fn check_not_empty(resistors: &[Ohms]) {
    assert!(!resistors.is_empty(), "need at least one resistor");
}

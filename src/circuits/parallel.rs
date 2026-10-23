// src/circuits/parallel.rs  ->  module `crate::circuits::parallel`

use super::check_not_empty; // bring the parent's private helper into scope
use crate::units::Ohms;

/// Parallel resistors: 1 / R_total = 1/R1 + 1/R2 + ...
pub fn total(resistors: &[Ohms]) -> Ohms {
    check_not_empty(resistors);

    let mut inverse_sum = 0.0;
    for r in resistors {
        inverse_sum += 1.0 / r.value();
    }
    Ohms::new(1.0 / inverse_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-9;

    #[test]
    fn two_equal_resistors_halve() {
        let r = [Ohms::new(100.0), Ohms::new(100.0)];
        assert!((total(&r).value() - 50.0).abs() < EPS);
    }

    #[test]
    fn three_resistors() {
        // 1 / (1/100 + 1/220 + 1/330) = 56.897 Ohm
        let r = [Ohms::new(100.0), Ohms::new(220.0), Ohms::new(330.0)];
        let expected = 1.0 / (1.0 / 100.0 + 1.0 / 220.0 + 1.0 / 330.0);
        assert!((total(&r).value() - expected).abs() < EPS);
    }

    #[test]
    fn total_is_smaller_than_smallest_resistor() {
        let r = [Ohms::new(10.0), Ohms::new(1_000_000.0)];
        assert!(total(&r).value() < 10.0);
    }

    #[test]
    fn single_resistor_is_unchanged() {
        assert!((total(&[Ohms::new(330.0)]).value() - 330.0).abs() < EPS);
    }

    #[test]
    fn zero_ohm_branch_shorts_everything() {
        let r = [Ohms::new(0.0), Ohms::new(1000.0)];
        assert_eq!(total(&r).value(), 0.0);
    }

    #[test]
    #[should_panic(expected = "need at least one resistor")]
    fn empty_list_panics() {
        total(&[]);
    }
}

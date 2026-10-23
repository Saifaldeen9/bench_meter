// src/circuits/series.rs  ->  module `crate::circuits::series`

use crate::units::Ohms; // absolute path: starts at the crate root

/// Series resistors: R_total = R1 + R2 + ...
pub fn total(resistors: &[Ohms]) -> Ohms {
    super::check_not_empty(resistors); // relative path: go up to `circuits`

    let mut sum = 0.0;
    for r in resistors {
        sum += r.value();
    }
    Ohms::new(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_three_resistors() {
        let r = [Ohms::new(100.0), Ohms::new(220.0), Ohms::new(330.0)];
        assert_eq!(total(&r).value(), 650.0);
    }

    #[test]
    fn single_resistor_is_unchanged() {
        assert_eq!(total(&[Ohms::new(470.0)]).value(), 470.0);
    }

    #[test]
    fn zero_ohm_links_add_nothing() {
        let r = [Ohms::new(0.0), Ohms::new(1000.0), Ohms::new(0.0)];
        assert_eq!(total(&r).value(), 1000.0);
    }

    #[test]
    #[should_panic(expected = "need at least one resistor")]
    fn empty_list_panics() {
        total(&[]);
    }
}

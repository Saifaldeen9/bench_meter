// src/units.rs  ->  module `crate::units`
// Small "newtype" structs so we never mix up volts, ohms, and amps.

/// Resistance in ohms. The field is PRIVATE (no `pub`),
/// so other modules must use `Ohms::new`, which checks the value.
#[derive(Debug, Clone, Copy)]
pub struct Ohms(f64);

impl Ohms {
    /// Create a resistance. Negative values make no sense, so we refuse them.
    pub fn new(value: f64) -> Ohms {
        assert!(value >= 0.0, "resistance cannot be negative");
        Ohms(value)
    }

    /// Read the raw number (read-only access to the private field).
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Voltage in volts. Any value is allowed, so the field is public.
#[derive(Debug, Clone, Copy)]
pub struct Volts(pub f64);

/// Current in amperes.
#[derive(Debug, Clone, Copy)]
pub struct Amps(pub f64);

/// Ohm's law: I = V / R
pub fn current(v: Volts, r: Ohms) -> Amps {
    Amps(v.0 / r.0) // inside `units` we may touch the private field `r.0`
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-9;

    #[test]
    fn ohms_new_stores_value() {
        assert_eq!(Ohms::new(220.0).value(), 220.0);
    }

    #[test]
    fn ohms_new_accepts_zero() {
        assert_eq!(Ohms::new(0.0).value(), 0.0);
    }

    #[test]
    #[should_panic(expected = "resistance cannot be negative")]
    fn ohms_new_rejects_negative() {
        Ohms::new(-1.0);
    }

    #[test]
    fn ohms_law_current() {
        // 5 V across 650 Ohm = 7.692 mA
        let i = current(Volts(5.0), Ohms::new(650.0));
        assert!((i.0 - 5.0 / 650.0).abs() < EPS);
    }

    #[test]
    fn zero_volts_gives_zero_current() {
        let i = current(Volts(0.0), Ohms::new(1000.0));
        assert_eq!(i.0, 0.0);
    }

    #[test]
    fn negative_volts_gives_negative_current() {
        let i = current(Volts(-12.0), Ohms::new(100.0));
        assert!((i.0 + 0.12).abs() < EPS);
    }

    #[test]
    fn zero_ohms_is_a_short_circuit() {
        // Dividing by zero ohms gives infinite current (a short circuit).
        let i = current(Volts(5.0), Ohms::new(0.0));
        assert!(i.0.is_infinite());
    }
}

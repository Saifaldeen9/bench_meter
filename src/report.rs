// src/report.rs  ->  module `crate::report`
// Turns numbers into nice text lines. Contains an INLINE child module.

use crate::units::{Amps, Ohms, Volts};

/// Visible anywhere inside this crate, but not to other crates.
pub(crate) const LABEL_WIDTH: usize = 9;

pub fn resistance_line(label: &str, r: Ohms) -> String {
    format!("{label:<width$}: {}", format::ohms(r), width = LABEL_WIDTH)
}

pub fn current_line(v: Volts, r: Ohms, i: Amps) -> String {
    format!(
        "{:<width$}: {:.1} V / {} = {}",
        "Current",
        v.0,
        format::ohms(r),
        format::milliamps(i),
        width = LABEL_WIDTH
    )
}

// An inline module: its body is written right here, in braces.
mod format {
    use crate::units::{Amps, Ohms};

    // `pub(super)`: only the parent module (`report`) may call these.
    pub(super) fn ohms(r: Ohms) -> String {
        let v = r.value();
        if v >= 1000.0 {
            format!("{:.2} kOhm", v / 1000.0)
        } else {
            format!("{:.1} Ohm", v)
        }
    }

    pub(super) fn milliamps(i: Amps) -> String {
        format!("{:.2} mA", i.0 * 1000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resistance_line_pads_label() {
        assert_eq!(
            resistance_line("Series", Ohms::new(650.0)),
            "Series   : 650.0 Ohm"
        );
    }

    #[test]
    fn ohms_below_1k_stay_in_ohms() {
        assert_eq!(format::ohms(Ohms::new(999.9)), "999.9 Ohm");
    }

    #[test]
    fn ohms_at_1k_switch_to_kohm() {
        assert_eq!(format::ohms(Ohms::new(1000.0)), "1.00 kOhm");
        assert_eq!(format::ohms(Ohms::new(4700.0)), "4.70 kOhm");
    }

    #[test]
    fn milliamps_formatting() {
        assert_eq!(format::milliamps(Amps(0.0123)), "12.30 mA");
        assert_eq!(format::milliamps(Amps(0.0)), "0.00 mA");
    }

    #[test]
    fn current_line_full_text() {
        let line = current_line(Volts(5.0), Ohms::new(650.0), Amps(5.0 / 650.0));
        assert_eq!(line, "Current  : 5.0 V / 650.0 Ohm = 7.69 mA");
    }

    #[test]
    fn long_label_is_not_truncated() {
        let line = resistance_line("Very long label", Ohms::new(1.0));
        assert!(line.starts_with("Very long label: "));
    }
}

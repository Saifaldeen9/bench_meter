# bench_meter

![CI](https://github.com/Saifaldeen9/bench_meter/actions/workflows/ci.yml/badge.svg)

`bench_meter` is a small Rust command-line tool that does the resistor math you normally scribble next to the breadboard: series and parallel totals and the current a supply pushes through them. It uses Rust "newtype" structs (`Ohms`, `Volts`, `Amps`) so the compiler stops you from mixing up units, and it is organised into clear modules to show how a real Rust crate is structured.

## Features

- **Series resistance**: `R_total = R1 + R2 + ...`
- **Parallel resistance**: `1 / R_total = 1/R1 + 1/R2 + ...`
- **Ohm's law current**: `I = V / R`
- **Type-safe units**: `Ohms::new` rejects negative resistance; `Volts` and `Amps` are separate types
- **Readable output**: automatic `Ohm` / `kOhm` and `mA` formatting
- **23 unit tests** covering normal values, zero-ohm shorts, empty input, and formatting boundaries

## Project structure

```text
bench_meter/
├── .github/workflows/ci.yml   # GitHub Actions: fmt + clippy + test
├── src/
│   ├── main.rs                # crate root / CLI entry point
│   ├── units.rs               # Ohms, Volts, Amps + Ohm's law
│   ├── circuits.rs            # circuit module + shared helper
│   ├── circuits/
│   │   ├── series.rs          # series total
│   │   └── parallel.rs        # parallel total
│   └── report.rs              # text formatting for the terminal
├── Cargo.toml
├── Cargo.lock
├── LICENSE
└── README.md
```

## Quick start

You need a Rust toolchain from [rustup.rs](https://rustup.rs).

```bash
cargo run                                   # run the bench meter
cargo test                                  # run all unit tests
cargo clippy --all-targets -- -D warnings   # lint (treat warnings as errors)
cargo fmt --all -- --check                  # check formatting
```

## Sample output

```text
$ cargo run
=== Bench Meter ===
Series   : 650.0 Ohm
Parallel : 56.9 Ohm
Current  : 5.0 V / 650.0 Ohm = 7.69 mA
```

## License

Released under the [MIT License](LICENSE). Copyright (c) 2026 Saifaldeen9.

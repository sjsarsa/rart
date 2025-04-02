# Rustic adaptive random testing algorithm testing

## Setup 

1. Install [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. Build project: `cargo build --release`

## Execute ART tests

**Test ART effectiveness** 

```
target/release/test_effectiveness
```

**Test ART efficiency** 

```
target/release/test_efficiency
```

The results will be saved under a `test-results` directory.

The ART test configurations are set in their respective Rust files in the `src/bin/` directory.

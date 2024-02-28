# WyRand-rs

[![CI](https://github.com/Bluefinger/wyrand-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Bluefinger/wyrand-rs/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](https://github.com/Bluefinger/wyrand-rs)
[![Cargo](https://img.shields.io/crates/v/wyrand.svg)](https://crates.io/crates/wyrand)
[![Documentation](https://docs.rs/wyrand/badge.svg)](https://docs.rs/wyrand)

A fast & portable non-cryptographic pseudorandom number generator written in Rust, and optionally, the hashing algorithm as well.

The implementations for both the PRNG and hasher are based on the final v4 C implementation [wyhash](https://github.com/wangyi-fudan/wyhash), a simple and fast hasher but **not** cryptographically secure. It's known to be extremely fast and performant while still having great statistical properties.

This crate provides both the v4.2 final implementation of the WyRand/WyHash algorithm (by default), or the older final v4 implementation. The two versions have different outputs due to changes in the algorithm and also with the constants used.

This crate can be used on its own or be integrated with `rand_core`/`rand`, and it is `no-std` compatible. Minimum compatible Rust version is 1.60. This crate is also implemented with no unsafe code via `#![forbid(unsafe_code)]`.

## Example

Generate a random value:

```rust
use wyrand::WyRand;

// Provide a seed to the PRNG
let mut rng = WyRand::new(Default::default());

let value = rng.rand();
```

## Features

The crate will always export `WyRand` and will do so when set as `default-features = false` in the Cargo.toml. By default, it will have the `rand_core`, `debug` & `v4_2` features enabled.

- **`rand_core`** - Enables support for `rand_core`, implementing `RngCore` & `SeedableRng` on `WyRand`.
- **`debug`** - Enables `core::fmt::Debug` implementation for `WyRand`/`WyHash`.
- **`serde1`** - Enables `Serialize` and `Deserialize` derives on `WyRand`.
- **`hash`** - Enables `core::hash::Hash` implementation for `WyRand`.
- **`wyhash`** - Enables `WyHash`, a fast & portable hashing algorithm. Based on the final v4 C implementation.
- **`v4_2`** - Switches the PRNG/Hashing algorithms to use the final v4.2 implementation. On by default. To force the algorithm to be the old final v4 one, you need to set `default-features = false`.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

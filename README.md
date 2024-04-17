# WyRand-rs

[![CI](https://github.com/Bluefinger/wyrand-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Bluefinger/wyrand-rs/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](https://github.com/Bluefinger/wyrand-rs)
[![Cargo](https://img.shields.io/crates/v/wyrand.svg)](https://crates.io/crates/wyrand)
[![Documentation](https://docs.rs/wyrand/badge.svg)](https://docs.rs/wyrand)

A fast & portable non-cryptographic pseudorandom number generator written in Rust, and optionally, the hashing algorithm as well.

The implementations for both the PRNG and hasher are based on the C reference implementation [wyhash](https://github.com/wangyi-fudan/wyhash), a simple and fast hasher but **not** cryptographically secure. It's known to be extremely fast and performant while still having great statistical properties.

This crate provides both the v4.2 final implementation of the WyRand/WyHash algorithm, or the older final v4 implementation. The two versions have different outputs due to changes in the algorithm and also with the constants used. By default, the newer final v4.2 algorithm will be provided. If there's a need to use the older version for compatibility reasons, the legacy versions of the PRNG and hasher are available behind the `legacy_v4` feature flag.

This crate can be used on its own or be integrated with `rand_core`/`rand`, and it is `no-std` compatible. Minimum compatible Rust version is 1.70. This crate is also implemented with no unsafe code via `#![forbid(unsafe_code)]`.

## Example

Generate a random value:

```rust
use wyrand::WyRand;

// Provide a seed to the PRNG
let mut rng = WyRand::new(Default::default());

let value = rng.rand();
```

## Features

The crate will always export `WyRand` and will do so when set as `default-features = false` in the Cargo.toml. By default, it will have the `rand_core`, `debug` features enabled.

- **`rand_core`** - Enables support for `rand_core`, implementing `RngCore` & `SeedableRng` on `WyRand`.
- **`debug`** - Enables `core::fmt::Debug` implementation for `WyRand`/`WyHash`.
- **`serde1`** - Enables `Serialize` and `Deserialize` derives on `WyRand`.
- **`hash`** - Enables `core::hash::Hash` implementation for `WyRand`.
- **`wyhash`** - Enables `WyHash`, a fast & portable hashing algorithm. Based on the final v4.2 C implementation.
- **`randomised_wyhash`** - Enables `RandomWyHashState`, a means to source a randomised state for `WyHash` for use in collections like `HashMap`/`HashSet`. Enables `wyhash` feature if it is not already enabled.
- **`fully_randomised_wyhash`** - Randomises not just the seed for `RandomWyHashState`, but also the secret. The new secret is generated once per runtime, and then is used for every subsequent new `WyHash` (with each `WyHash` instance having its own unique seed). Enables `randomised_wyhash` if not already enabled, and requires `std` environments.
- **`threadrng_wyhash`** - Enables sourcing entropy from `rand`'s `thread_rng()` method. Much quicker than `getrandom`. Enables `randomised_wyhash` if not already enabled. Requires `std` environments.
- **`legacy_v4`** - Exposes the legacy PRNG/Hashing algorithms.

## Building for WASM/Web

If you are using `WyRand` with `rand_core` and/or `WyHash` with `randomised_wyhash` then for building for the web/WASM, you'll need to configure `getrandom` to make use of the browser APIs in order to source entropy from. Add the following to your project `Cargo.toml` if your WASM builds target the web:

```toml
[target.'cfg(all(target_arch = "wasm32", target_os = "unknown"))'.dependencies]
getrandom = { version = "0.2", features = ["js"] }
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

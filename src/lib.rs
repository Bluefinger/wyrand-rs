//! A fast & portable non-cryptographic pseudorandom number generator written in Rust.
//!
//! The implementation is based on [wyhash](https://github.com/wangyi-fudan/wyhash), a
//! simple and fast hasher but **not**cryptographically secure. It's known to be extremely
//! fast and performant while still having great statistical properties.
//!
//! This crate can be used on its own or be integrated with `rand_core`/`rand`, and it is
//! `no-std` compatible. Minimum compatible Rust version is 1.60.
//!
//! # Example
//!
//! Generate a random value:
//!
//! ```rust
//! use wyrand::WyRand;
//!
//! // Provide a seed to the PRNG
//! let mut rng = WyRand::new(Default::default());
//!
//! let value = rng.rand();
//! ```
//!
//! # Features
//!
//! The  crate will always export [`WyRand`] and will do so when set as
//! `default-features = false` in the  Cargo.toml. By default, it will have the`rand_core`
//! & `debug` features enabled.
//!
//! * **`rand_core`** - Enables support for `rand_core`, implementing `RngCore` &
//!   `SeedableRng` on [`WyRand`].
//! * **`debug`** - Enables [`core::fmt::Debug`] implementation for [`WyRand`].
//! * **`serde1`** - Enables `Serialize` and `Deserialize` derives on [`WyRand`].
//! * **`hash`** - Enables [`core::hash::Hash`] implementation for [`WyRand`].
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

mod constants;
#[cfg(feature = "wyhash")]
mod hasher;
mod rand;
mod utils;

#[cfg(feature = "wyhash")]
pub use hasher::WyHash;
pub use rand::WyRand;

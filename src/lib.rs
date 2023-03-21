//! A fast & portable non-cryptographic pseudorandom number generator written in Rust.
//!
//! The implementation is based on [wyhash](https://github.com/wangyi-fudan/wyhash), a simple and fast hasher but **not** cryptographically secure. It's known to be extremely fast and performant while still having great statistical properties.
//!
//! This crate can be used on its own or be integrated with `rand_core`/`rand`, and it is `no-std` compatible. Minimum compatible Rust version is 1.60.
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
//! The  crate will always export [`WyRand`] and will do so when set as `default-features = false` in the Cargo.toml. By default, it will have the `rand_core` & `debug` features enabled.
//!
//! * **`rand_core`** - Enables support for `rand_core`, implementing `RngCore` &
//!   `SeedableRng` on [`WyRand`].
//! * **`debug`** - Enables [`core::fmt::Debug`] implementation for [`WyRand`].
//! * **`serde1`** - Enables `Serialize` and `Deserialize` derives on [`WyRand`].
#![warn(missing_docs)]
#![no_std]
#[cfg(feature = "debug")]
use core::fmt::Debug;

#[cfg(feature = "rand_core")]
use rand_core::{impls::fill_bytes_via_next, RngCore, SeedableRng};

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

/// A Pseudorandom Number generator, powered by the `wyrand` algorithm.
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct WyRand {
    state: u64,
}

impl WyRand {
    /// Creates a new [`WyRand`] instance with the provided seed. Be sure
    /// to obtain the seed value from a good entropy source, either from
    /// hardware, OS source, or from a suitable crate, like `getrandom`.
    #[inline]
    #[must_use]
    pub fn new(state: u64) -> Self {
        Self { state }
    }

    /// Generates a random [`u64`] value and advances the PRNG state.
    #[inline]
    pub fn rand(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0xa076_1d64_78bd_642f);
        let t = u128::from(self.state).wrapping_mul(u128::from(self.state ^ 0xe703_7ed1_a0b4_28db));
        (t.wrapping_shr(64) ^ t) as u64
    }
}

#[cfg(feature = "debug")]
impl Debug for WyRand {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("WyRand").finish()
    }
}

#[cfg(feature = "rand_core")]
impl RngCore for WyRand {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.rand() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.rand()
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        fill_bytes_via_next(self, dest);
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[cfg(feature = "rand_core")]
impl SeedableRng for WyRand {
    type Seed = [u8; core::mem::size_of::<u64>()];

    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(u64::from_ne_bytes(seed))
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::format;

    use super::*;

    #[cfg(feature = "debug")]
    #[test]
    fn no_leaking_debug() {
        let rng = WyRand::new(Default::default());

        assert_eq!(format!("{rng:?}"), "WyRand");
    }

    #[test]
    fn clone_rng() {
        let rng = WyRand::new(Default::default());

        let mut cloned = rng.clone();

        // Should be the same internal state after cloning
        assert_eq!(
            &rng, &cloned,
            "the two RNG instances are not the same after cloning"
        );

        cloned.rand();

        // Should no longer have the same internal state after generating a random number
        assert_ne!(
            &rng, &cloned,
            "the two RNG instances are the same after one was used"
        );
    }

    #[cfg(feature = "rand_core")]
    #[test]
    fn rand_core_integration() {
        let mut rng = WyRand::from_seed(Default::default());

        fn rand_generic<R: RngCore>(mut r: R) -> u32 {
            r.next_u32()
        }

        fn rand_dyn(r: &mut dyn RngCore) -> u32 {
            r.next_u32()
        }

        assert_eq!(rand_generic(&mut rng), 2405016974);
        assert_eq!(rand_dyn(&mut rng), 4283336045);
    }

    #[cfg(all(feature = "serde1", feature = "debug"))]
    #[test]
    fn serde_tokens() {
        use serde_test::{assert_tokens, Token};

        let seed = 12345;
        let rng = WyRand::new(seed);

        assert_tokens(
            &rng,
            &[
                Token::Struct {
                    name: "WyRand",
                    len: 1,
                },
                Token::BorrowedStr("state"),
                Token::U64(seed),
                Token::StructEnd,
            ],
        );
    }
}

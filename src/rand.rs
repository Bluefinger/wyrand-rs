#[cfg(feature = "debug")]
use core::fmt::Debug;

use crate::constants::{WY0, WY1};
#[cfg(feature = "rand_core")]
use rand_core::{impls::fill_bytes_via_next, RngCore, SeedableRng};

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};
use crate::utils::wymix;

/// A Pseudorandom Number generator, powered by the `wyrand` algorithm.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hash", derive(Hash))]
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
        self.state = self.state.wrapping_add(WY0);
        wymix(self.state, self.state ^ WY1)
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

    use super::*;

    #[cfg(feature = "debug")]
    #[test]
    fn no_leaking_debug() {
        use alloc::format;

        let rng = WyRand::new(Default::default());

        assert_eq!(
            format!("{rng:?}"),
            "WyRand",
            "Debug should not be leaking internal state"
        );
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
        fn rand_generic<R: RngCore>(mut r: R) -> u32 {
            r.next_u32()
        }

        fn rand_dyn(r: &mut dyn RngCore) -> u32 {
            r.next_u32()
        }

        let mut rng = WyRand::from_seed(Default::default());

        assert_eq!(rand_generic(&mut rng), 2_405_016_974);
        assert_eq!(rand_dyn(&mut rng), 4_283_336_045);
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

    #[cfg(feature = "hash")]
    #[allow(deprecated)]
    #[test]
    fn hash() {
        use core::hash::{Hash, Hasher, SipHasher};

        let rng = WyRand::new(123);
        let state: u64 = 123;

        let mut hasher = SipHasher::default();
        rng.hash(&mut hasher);
        let hashed_rng = hasher.finish();

        let mut hasher = SipHasher::default();
        state.hash(&mut hasher);
        let hashed_state = hasher.finish();

        assert_eq!(hashed_rng, hashed_state);
    }
}

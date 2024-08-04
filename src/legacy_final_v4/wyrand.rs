#[cfg(feature = "debug")]
use core::fmt::Debug;

use super::constants::{WY0, WY1};
#[cfg(feature = "rand_core")]
use rand_core::{impls::fill_bytes_via_next, RngCore, SeedableRng, TryRngCore};

use crate::utils::wymix;
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

/// A Pseudorandom Number generator, powered by the `wyrand` algorithm. This generator
/// is based on the legacy final v4 reference implementation.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hash", derive(Hash))]
#[repr(transparent)]
pub struct WyRandLegacy {
    state: u64,
}

impl WyRandLegacy {
    /// Creates a new [`WyRandLegacy`] instance with the provided seed. Be sure
    /// to obtain the seed value from a good entropy source, either from
    /// hardware, OS source, or from a suitable crate, like `getrandom`.
    #[inline]
    #[must_use]
    pub const fn new(state: u64) -> Self {
        Self { state }
    }

    /// Generates a random [`u64`] value and advances the PRNG state.
    #[inline]
    pub fn rand(&mut self) -> u64 {
        let (value, state) = Self::gen_u64(self.state);
        self.state = state;
        value
    }

    /// Const [`WyRandLegacy`] generator. Generates and returns a random [`u64`] value first
    /// and then the advanced state second.
    /// ```
    /// use wyrand::legacy_final_v4::WyRandLegacy;
    ///
    /// let seed = 123;
    ///
    /// let (random_value, new_state) = WyRandLegacy::gen_u64(seed);
    ///
    /// assert_ne!(random_value, 0);
    /// // The original seed now no longer matches the new state.
    /// assert_ne!(new_state, seed);
    /// ```
    #[inline(always)]
    pub const fn gen_u64(mut seed: u64) -> (u64, u64) {
        seed = seed.wrapping_add(WY0);
        (wymix(seed, seed ^ WY1), seed)
    }
}

#[cfg(feature = "debug")]
impl Debug for WyRandLegacy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("WyRandLegacy").finish()
    }
}

#[cfg(feature = "rand_core")]
impl RngCore for WyRandLegacy {
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
}

#[cfg(feature = "rand_core")]
impl SeedableRng for WyRandLegacy {
    type Seed = [u8; core::mem::size_of::<u64>()];

    #[inline]
    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(u64::from_ne_bytes(seed))
    }

    #[inline]
    fn from_rng(mut rng: impl RngCore) -> Self {
        Self::new(rng.next_u64())
    }

    #[inline]
    fn try_from_rng<R: TryRngCore>(mut rng: R) -> Result<Self, R::Error> {
        Ok(Self::new(rng.try_next_u64()?))
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

        let rng = WyRandLegacy::new(Default::default());

        assert_eq!(
            format!("{rng:?}"),
            "WyRandLegacy",
            "Debug should not be leaking internal state"
        );
    }

    #[test]
    fn clone_rng() {
        let rng = WyRandLegacy::new(Default::default());

        let mut cloned = rng.clone();

        // Should be the same internal state after cloning
        assert_eq!(
            &rng.state, &cloned.state,
            "the two RNG instances are not the same after cloning"
        );

        cloned.rand();

        // Should no longer have the same internal state after generating a random number
        assert_ne!(
            &rng.state, &cloned.state,
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

        let mut rng = WyRandLegacy::from_seed(Default::default());

        assert_eq!(rand_generic(&mut rng), 2_405_016_974);
        assert_eq!(rand_dyn(&mut rng), 4_283_336_045);
    }

    #[cfg(feature = "rand_core")]
    #[test]
    fn rand_core_from_rng() {
        let mut source = WyRandLegacy::from_seed(Default::default());

        let mut rng = WyRandLegacy::from_rng(&mut source);

        assert_eq!(rng.next_u32(), 4242651740);
    }

    #[cfg(all(feature = "serde1", feature = "debug"))]
    #[test]
    fn serde_tokens() {
        use serde_test::{assert_tokens, Token};

        let seed = 12345;
        let rng = WyRandLegacy::new(seed);

        assert_tokens(
            &rng,
            &[
                Token::Struct {
                    name: "WyRandLegacy",
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

        let rng = WyRandLegacy::new(123);
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

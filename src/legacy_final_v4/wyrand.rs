#[cfg(feature = "rand_core")]
use core::convert::Infallible;
#[cfg(feature = "debug")]
use core::fmt::Debug;

use super::constants::{WY0, WY1};
#[cfg(feature = "rand_core")]
use rand_core::{Rng, SeedableRng, TryRng};

use crate::utils::wymix;
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

/// A Pseudorandom Number generator, powered by the `wyrand` algorithm. This generator
/// is based on the legacy final v4 reference implementation.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hash", derive(Hash))]
pub struct WyRandLegacy {
    state: u64,
}

impl WyRandLegacy {
    /// Creates a new [`WyRandLegacy`] instance with the provided seed. Be sure
    /// to obtain the seed value from a good entropy source, either from
    /// hardware, OS source, or from a suitable crate, like `getrandom`.
    ///
    /// If the seed value is known to be small/weak, then use `rand_core`'s
    /// `SeedableRng::seed_from_u64` method instead.
    #[inline]
    #[must_use]
    pub const fn new(state: u64) -> Self {
        Self { state }
    }

    /// Generates a random [`u64`] value and advances the PRNG state.
    #[inline]
    #[must_use]
    pub const fn rand(&mut self) -> u64 {
        Self::gen_u64(&mut self.state)
    }

    /// Const [`WyRandLegacy`] generator. Generates and returns a random [`u64`] value,
    /// after advancing the state mutable reference.
    #[inline(always)]
    const fn gen_u64(state: &mut u64) -> u64 {
        *state = state.wrapping_add(WY0);
        wymix(*state, *state ^ WY1)
    }
}

#[cfg(feature = "debug")]
impl Debug for WyRandLegacy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("WyRandLegacy").finish()
    }
}

#[cfg(feature = "rand_core")]
impl TryRng for WyRandLegacy {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.rand() as u32)
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(self.rand())
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        rand_core::utils::fill_bytes_via_next_word(dst, || self.try_next_u64())
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
    fn seed_from_u64(mut state: u64) -> Self {
        Self {
            state: Self::gen_u64(&mut state),
        }
    }

    #[inline]
    fn from_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self::new(rng.next_u64())
    }

    #[inline]
    fn try_from_rng<R: TryRng + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
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

        let _ = cloned.rand();

        // Should no longer have the same internal state after generating a random number
        assert_ne!(
            &rng.state, &cloned.state,
            "the two RNG instances are the same after one was used"
        );
    }

    #[cfg(feature = "rand_core")]
    #[test]
    fn rand_core_integration() {
        fn rand_generic<R: Rng>(mut r: R) -> u32 {
            r.next_u32()
        }

        fn rand_dyn(r: &mut dyn Rng) -> u32 {
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
        use serde_test::{Token, assert_tokens};

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

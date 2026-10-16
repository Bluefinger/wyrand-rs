#[cfg(feature = "rand_core")]
use core::convert::Infallible;
#[cfg(feature = "debug")]
use core::fmt::Debug;

use crate::final_v4_2::constants::{WY0, WY1};
#[cfg(feature = "rand_core")]
use rand_core::{Rng, SeedableRng, TryRng};

use crate::utils::wymix;
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

/// A Pseudorandom Number generator, powered by the `wyrand` algorithm. This generator
/// is based on the asymmetric construction of the reference implementation, whose
/// output multiply is fed from *two* consecutive counter states instead of one. This improvement
/// was proposed by Sebastiano Vigna and Reiner Pope
/// ([wyhash/issue#130](https://github.com/wangyi-fudan/wyhash/issues/130#issuecomment-4835746792) and
/// [wyhash/issue#156](https://github.com/wangyi-fudan/wyhash/issues/156)).
///
/// Plain `WyRand` mixes a single state, `wymix(state, state ^ XOR)`. Because `wymix` is
/// commutative and `x -> x ^ XOR` is a fixed-point-free involution, `f(x) == f(x ^ XOR)`: the output
/// filter is exactly 2-to-1, so it can only ever reach `1 - e^(-1/2) ≈ 39.3%` of its output space,
/// where a well-behaved generator reaches `1 - 1/e ≈ 63.2%`. [`WyRandA`] multiplies the *old* (xored)
/// state by the *new* (incremented) state, which is not a commutative function of a single value and
/// as such removes the symmetry and restores the full `~63.2%` coverage at identical speed.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hash", derive(Hash))]
pub struct WyRandA {
    state: u64,
}

impl WyRandA {
    /// Creates a new [`WyRandA`] instance with the provided seed. Be sure
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
    #[inline(always)]
    #[must_use]
    pub const fn rand(&mut self) -> u64 {
        Self::gen_u64(&mut self.state)
    }

    /// Const [`WyRandA`] generator. Generates and returns a random [`u64`] value,
    /// after advancing the state mutable reference.
    #[inline(always)]
    #[must_use]
    const fn gen_u64(state: &mut u64) -> u64 {
        let old_xored_state = *state ^ WY1;
        *state = state.wrapping_add(WY0);
        wymix(*state, old_xored_state)
    }
}

#[cfg(feature = "debug")]
impl Debug for WyRandA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("WyRandA").finish()
    }
}

#[cfg(feature = "rand_core")]
impl TryRng for WyRandA {
    type Error = Infallible;

    #[inline]
    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.rand() as u32)
    }

    #[inline]
    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(self.rand())
    }

    #[inline]
    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        rand_core::utils::fill_bytes_via_next_word(dst, || self.try_next_u64())
    }
}

#[cfg(feature = "rand_core")]
impl SeedableRng for WyRandA {
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

        let rng = WyRandA::new(Default::default());

        assert_eq!(
            format!("{rng:?}"),
            "WyRandA",
            "Debug should not be leaking internal state"
        );
    }

    #[test]
    fn clone_rng() {
        let rng = WyRandA::new(Default::default());

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

        let mut rng = WyRandA::from_seed(Default::default());

        assert_eq!(rand_generic(&mut rng), 3_349_934_249);
        assert_eq!(rand_dyn(&mut rng), 447_996_333);
    }

    #[cfg(feature = "rand_core")]
    #[test]
    fn rand_core_from_rng() {
        let mut source = WyRandA::from_seed(Default::default());

        let mut rng = WyRandA::from_rng(&mut source);

        assert_eq!(rng.next_u32(), 154783579);
    }

    #[cfg(all(feature = "serde1", feature = "debug"))]
    #[test]
    fn serde_tokens() {
        use serde_test::{Token, assert_tokens};

        let seed = 12345;
        let rng = WyRandA::new(seed);

        assert_tokens(
            &rng,
            &[
                Token::Struct {
                    name: "WyRandA",
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

        let rng = WyRandA::new(123);
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

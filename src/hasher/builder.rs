use core::{hash::BuildHasher, mem::MaybeUninit};

#[cfg(feature = "debug")]
use core::fmt::Debug;

use getrandom::getrandom_uninit;

use crate::WyHash;

#[cfg_attr(docsrs, doc(cfg(feature = "randomised_wyhash")))]
#[derive(Clone, Copy)]
/// Randomised state constructor for [`WyHash`]. This builder will source entropy in order
/// to provide random seeds for [`WyHash`]. This will yield a hasher with not just a random
/// seed, but also a new random secret, granting extra protection against DOS and prediction
/// attacks.
pub struct RandomWyHashState(u64, u64);

impl RandomWyHashState {
    /// Create a new [`RandomWyHashState`] instance. Calling this method will attempt to
    /// draw entropy from hardware/OS sources.
    ///
    /// # Panics
    ///
    /// This method will panic if it was unable to source enough entropy.
    ///
    /// # Examples
    ///
    /// ```
    /// use wyrand::RandomWyHashState;
    /// use core::hash::BuildHasher;
    ///
    /// let s = RandomWyHashState::new();
    ///
    /// let mut hasher = s.build_hasher(); // Creates a WyHash instance with random state
    /// ```
    #[must_use]
    pub fn new() -> Self {
        // Don't bother zeroing as we will initialise this with random data. If the initialisation fails
        // for any reason, we will panic instead of trying to continue with a fully or partially
        // uninitialised buffer. This ensures the whole process is safe without the need to use an
        // unsafe block.
        let mut bytes = [MaybeUninit::<u8>::uninit(); core::mem::size_of::<u64>() * 2];

        let bytes = getrandom_uninit(&mut bytes)
            .expect("Failed to source entropy for WyHash randomised state");

        let (first, second) = bytes.split_at(core::mem::size_of::<u64>());

        let first = u64::from_ne_bytes(first.try_into().unwrap());
        let second = u64::from_ne_bytes(second.try_into().unwrap());

        Self(first, second)
    }
}

impl BuildHasher for RandomWyHashState {
    type Hasher = WyHash;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        WyHash::new(self.0, self.1)
    }
}

impl Default for RandomWyHashState {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "debug")]
impl Debug for RandomWyHashState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RandomisedWyHashBuilder")
            .finish_non_exhaustive()
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

        let builder = RandomWyHashState::default();

        assert_eq!(
            format!("{builder:?}"),
            "RandomisedWyHashBuilder { .. }",
            "Debug should not be leaking internal state"
        );
    }

    #[test]
    fn randomised_builder_states() {
        let builder1 = RandomWyHashState::new();
        let builder2 = RandomWyHashState::new();

        // The two builders' internal states are different to each other
        assert_ne!(&builder1.0, &builder2.0);
        assert_ne!(&builder1.1, &builder2.1);

        // Each builder's internal state should not be the same (hopefully).
        // It is more likely that we have not initialised things correctly than
        // to have the entropy source output the same bits for both fields.
        assert_ne!(&builder1.0, &builder1.1);
        assert_ne!(&builder2.0, &builder2.1);
    }
}

use core::hash::BuildHasher;

#[cfg(feature = "debug")]
use core::fmt::Debug;

use crate::WyHash;

#[cfg_attr(docsrs, doc(cfg(feature = "randomised_wyhash")))]
#[derive(Clone, Copy)]
#[repr(align(8))]
/// Randomised state constructor for [`WyHash`]. This builder will source entropy in order
/// to provide random seeds for [`WyHash`]. This will yield a hasher with not just a random
/// seed, but also a new random secret, granting extra protection against DOS and prediction
/// attacks.
pub struct RandomWyHashState {
    #[cfg(feature = "fully_randomised_wyhash")]
    state: [u8; 16],
    #[cfg(not(feature = "fully_randomised_wyhash"))]
    state: [u8; 8],
}

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
        #[cfg(feature = "fully_randomised_wyhash")]
        const SIZE: usize = core::mem::size_of::<u64>() * 2;
        #[cfg(not(feature = "fully_randomised_wyhash"))]
        const SIZE: usize = core::mem::size_of::<u64>();

        let mut state = [0; SIZE];

        #[cfg(not(feature = "threadrng_wyhash"))]
        {
            getrandom::getrandom(&mut state)
                .expect("Failed to source entropy for WyHash randomised state");
        }
        #[cfg(feature = "threadrng_wyhash")]
        {
            use rand::RngCore;

            rand::thread_rng().fill_bytes(&mut state);
        }

        Self { state }
    }
}

impl BuildHasher for RandomWyHashState {
    type Hasher = WyHash;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        #[cfg(feature = "fully_randomised_wyhash")]
        {
            let (first_seed, second_seed) = self.state.split_at(core::mem::size_of::<u64>());

            let first_seed = u64::from_ne_bytes(first_seed.try_into().unwrap());
            let second_seed = u64::from_ne_bytes(second_seed.try_into().unwrap());

            WyHash::new(first_seed, second_seed)
        }
        #[cfg(not(feature = "fully_randomised_wyhash"))]
        {
            let seed = u64::from_ne_bytes(self.state);

            WyHash::new_with_default_secret(seed)
        }
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
        assert_ne!(&builder1.state, &builder2.state);
    }
}

use core::hash::BuildHasher;

#[cfg(feature = "debug")]
use core::fmt::Debug;

#[cfg(feature = "fully_randomised_wyhash")]
use std::sync::OnceLock;

use crate::WyHash;

#[cfg(feature = "fully_randomised_wyhash")]
static SECRET: OnceLock<[u64; 4]> = OnceLock::new();

#[inline]
fn get_random_u64() -> u64 {
    #[cfg(not(feature = "threadrng_wyhash"))]
    {
        const SIZE: usize = core::mem::size_of::<u64>();

        let mut state = [0; SIZE];

        // Don't bother trying to handle the result. If we can't obtain
        // entropy with getrandom, then there is no hope and we might as
        // well panic. It is up to the user to ensure getrandom is configured
        // correctly for their platform.
        getrandom::getrandom(&mut state)
            .expect("Failed to source entropy for WyHash randomised state");

        u64::from_ne_bytes(state)
    }
    #[cfg(feature = "threadrng_wyhash")]
    {
        use rand::RngCore;

        // This is faster than doing `.fill_bytes()`. User-space entropy goes brrr.
        rand::thread_rng().next_u64()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "randomised_wyhash")))]
#[derive(Clone, Copy)]
/// Randomised state constructor for [`WyHash`]. This builder will source entropy in order
/// to provide random seeds for [`WyHash`]. If the `fully_randomised_wyhash` feature is enabled,
/// this will yield a hasher with not just a random seed, but also a new random secret,
/// granting extra protection against DOS and prediction attacks.
pub struct RandomWyHashState {
    state: u64,
    secret: [u64; 4],
}

impl RandomWyHashState {
    /// Create a new [`RandomWyHashState`] instance. Calling this method will attempt to
    /// draw entropy from hardware/OS sources. If `fully_randomised_wyhash` feature is enabled,
    /// then it will use a randomised `secret` as well, otherwise it uses the default wyhash constants. 
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
    #[inline]
    pub fn new() -> Self {
        #[cfg(feature = "fully_randomised_wyhash")]
        use crate::hasher::secret::make_secret;

        #[cfg(not(feature = "fully_randomised_wyhash"))]
        use crate::constants::{WY0, WY1, WY2, WY3};

        #[cfg(feature = "fully_randomised_wyhash")]
        let secret = *SECRET.get_or_init(|| make_secret(get_random_u64()));
        #[cfg(not(feature = "fully_randomised_wyhash"))]
        let secret = [WY0, WY1, WY2, WY3];

        Self::new_with_secret(secret)
    }

    /// Create a new [`RandomWyHashState`] instance with a provided secret. Calling this method
    /// will attempt to draw entropy from hardware/OS sources, and assumes the user provided the
    /// secret via [`super::secret::make_secret`].
    ///
    /// # Panics
    ///
    /// This method will panic if it was unable to source enough entropy.
    ///
    /// # Examples
    ///
    /// ```
    /// use wyrand::{RandomWyHashState, make_secret};
    /// use core::hash::BuildHasher;
    ///
    /// let s = RandomWyHashState::new_with_secret(make_secret(42));
    ///
    /// let mut hasher = s.build_hasher(); // Creates a WyHash instance with random state
    /// ```
    #[must_use]
    #[inline]
    pub fn new_with_secret(secret: [u64; 4]) -> Self {
        Self {
            state: get_random_u64(),
            secret,
        }
    }
}

impl BuildHasher for RandomWyHashState {
    type Hasher = WyHash;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        WyHash::new_with_secret(self.state, self.secret)
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

        // The two builders' internal secrets are the same to each other
        assert_eq!(&builder1.secret, &builder2.secret);

        // When fully randomised, the generated secrets should not be the
        // same as the default secret.
        #[cfg(feature = "fully_randomised_wyhash")]
        {
            use crate::constants::{WY0, WY1, WY2, WY3};

            let default_secret = [WY0, WY1, WY2, WY3];

            assert_ne!(&builder1.secret, &default_secret);
            assert_ne!(&builder2.secret, &default_secret);
        }
    }
}

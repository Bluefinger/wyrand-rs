use core::hash::BuildHasher;

#[cfg(feature = "debug")]
use core::fmt::Debug;

#[cfg(feature = "fully_randomised_wyhash")]
use std::sync::OnceLock;

use super::WyHashLegacy;

use super::secret::LegacySecret;

#[cfg(feature = "fully_randomised_wyhash")]
static SECRET: OnceLock<LegacySecret> = OnceLock::new();

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

#[derive(Clone)]
/// Randomised state constructor for [`WyHashLegacy`]. This builder will source entropy in order
/// to provide random seeds for [`WyHashLegacy`]. If the `fully_randomised_wyhash` feature is enabled,
/// this will yield a hasher with not just a random seed, but also a new random secret,
/// granting extra protection against DOS and prediction attacks.
pub struct RandomWyHashLegacyState {
    state: u64,
    secret: LegacySecret,
}

impl RandomWyHashLegacyState {
    /// Create a new [`RandomWyHashLegacyState`] instance. Calling this method will attempt to
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
    /// use wyrand::legacy_final_v4::RandomWyHashLegacyState;
    /// use core::hash::BuildHasher;
    ///
    /// let s = RandomWyHashLegacyState::new();
    ///
    /// let mut hasher = s.build_hasher(); // Creates a WyHash instance with random state
    /// ```
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        #[cfg(not(feature = "fully_randomised_wyhash"))]
        use super::constants::{WY0, WY1, WY2, WY3};
        #[cfg(feature = "fully_randomised_wyhash")]
        use super::secret::make_secret_legacy;

        #[cfg(feature = "fully_randomised_wyhash")]
        let secret = SECRET
            .get_or_init(|| make_secret_legacy(get_random_u64()))
            .clone();
        #[cfg(not(feature = "fully_randomised_wyhash"))]
        let secret = LegacySecret::new(WY0, WY1, WY2, WY3);

        Self::new_with_secret(secret)
    }

    /// Create a new [`RandomWyHashLegacyState`] instance with a provided secret. Calling this method
    /// will attempt to draw entropy from hardware/OS sources, and assumes the user provided the
    /// secret via [`WyHashLegacy::make_secret`].
    ///
    /// # Panics
    ///
    /// This method will panic if it was unable to source enough entropy.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::hash::BuildHasher;
    /// use wyrand::legacy_final_v4::{RandomWyHashLegacyState, WyHashLegacy};
    ///
    /// let s = RandomWyHashLegacyState::new_with_secret(WyHashLegacy::make_secret(42));
    ///
    /// let mut hasher = s.build_hasher(); // Creates a WyHash instance with random state
    /// ```
    #[must_use]
    #[inline]
    pub fn new_with_secret(secret: LegacySecret) -> Self {
        Self {
            state: get_random_u64(),
            secret,
        }
    }
}

impl BuildHasher for RandomWyHashLegacyState {
    type Hasher = WyHashLegacy;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        WyHashLegacy::new_with_secret(self.state, self.secret.clone())
    }
}

impl Default for RandomWyHashLegacyState {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "debug")]
impl Debug for RandomWyHashLegacyState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RandomisedWyHashLegacyState")
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

        let builder = RandomWyHashLegacyState::default();

        assert_eq!(
            format!("{builder:?}"),
            "RandomisedWyHashLegacyState { .. }",
            "Debug should not be leaking internal state"
        );
    }

    #[test]
    fn randomised_builder_states() {
        let builder1 = RandomWyHashLegacyState::new();
        let builder2 = RandomWyHashLegacyState::new();

        // The two builders' internal states are different to each other
        assert_ne!(&builder1.state, &builder2.state);

        // The two builders' internal secrets are the same to each other
        assert_eq!(&builder1.secret, &builder2.secret);

        // When fully randomised, the generated secrets should not be the
        // same as the default secret.
        #[cfg(feature = "fully_randomised_wyhash")]
        {
            use super::super::constants::{WY0, WY1, WY2, WY3};

            let default_secret = LegacySecret::new(WY0, WY1, WY2, WY3);

            assert_ne!(&builder1.secret, &default_secret);
            assert_ne!(&builder2.secret, &default_secret);
        }
    }
}

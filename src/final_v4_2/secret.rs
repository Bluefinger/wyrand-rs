use crate::{constants::C_VALUES, utils::check_for_valid_secret_value, WyRand};

#[cfg(feature = "debug")]
use core::fmt::Debug;

use super::primes::is_prime;

#[derive(Clone, PartialEq, Eq)]
#[repr(align(32))]
/// A wrapper struct for containing generated secrets to be used by the wyhash algorithm. Ensures it can't be used
/// incorrectly, and can only be constructed by [`super::WyHash::make_secret`].
pub struct Secret([u64; 4]);

impl Secret {
    #[must_use]
    #[inline(always)]
    pub(super) const fn new(first: u64, second: u64, third: u64, fourth: u64) -> Self {
        Self([first, second, third, fourth])
    }

    #[inline(always)]
    pub(super) const fn first(&self) -> u64 {
        self.0[0]
    }

    #[inline(always)]
    pub(super) const fn second(&self) -> u64 {
        self.0[1]
    }

    #[inline(always)]
    pub(super) const fn third(&self) -> u64 {
        self.0[2]
    }

    #[inline(always)]
    pub(super) const fn fourth(&self) -> u64 {
        self.0[3]
    }
}

#[cfg(feature = "debug")]
impl Debug for Secret {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Secret").finish()
    }
}

/// Generate new secret for wyhash. Takes a seed value and outputs an array of 4 suitable `u64` constants
/// for use with the hasher. The PRNG will always use the default constants provided.
pub(super) const fn make_secret(mut seed: u64) -> Secret {
    let mut secret: [u64; 4] = [0; 4];
    let mut i: usize = 0;

    while i < secret.len() {
        let mut ok: bool = false;

        while !ok {
            secret[i] = 0;
            let mut j: usize = 0;

            while j < 64 {
                // WyRand... but const!
                let (value, new_state) = WyRand::gen_u64(seed);
                seed = new_state;
                let random_index = (value as usize) % C_VALUES.len();
                secret[i] |= (C_VALUES[random_index] as u64) << j;
                j += 8;
            }

            ok = check_for_valid_secret_value(i, &secret);

            if ok && !is_prime(secret[i]) {
                ok = false;
            }
        }

        i += 1;
    }

    Secret(secret)
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    #[cfg(feature = "debug")]
    #[test]
    fn no_leaking_debug() {
        use alloc::format;

        let secret = Secret::new(
            0x39d43c5c4e3a724b,
            0x6596e14753cca38b,
            0xc68d954b2b339353,
            0x96b4a6e45c65aa55,
        );

        assert_eq!(
            format!("{secret:?}"),
            "Secret",
            "Debug should not be leaking internal state"
        );
    }

    #[cfg(feature = "debug")]
    #[test]
    fn generate_expected_secrets() {
        let test_cases: [u64; 4] = [0, 3, 6, 42];
        let expected_results: [Secret; 4] = [
            Secret::new(
                0x39d43c5c4e3a724b,
                0x6596e14753cca38b,
                0xc68d954b2b339353,
                0x96b4a6e45c65aa55,
            ),
            Secret::new(
                0xa3743ca35956ac59,
                0x65b1b8e8558b72c5,
                0x78cad4b8c98ea535,
                0x561d59965a4baa27,
            ),
            Secret::new(
                0x993c394d599a9a2b,
                0x535c4d3c9ae1a91d,
                0x72b2356a3cc6f0a5,
                0x5a6c8e1b6c2e4da9,
            ),
            Secret::new(
                0x8b4be21b934dc6a3,
                0x9a0f72f0e81b6969,
                0x99746a47f066331b,
                0xccb8b85a99aaa9b1,
            ),
        ];

        test_cases
            .into_iter()
            .zip(expected_results)
            .for_each(|(seed, expected)| {
                let result = make_secret(seed);

                assert_eq!(&result, &expected, "Failed secret for seed: {}", seed);
            });
    }
}

use crate::constants::C_VALUES;

use super::WyRandLegacy;

#[cfg(feature = "debug")]
use core::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
/// A wrapper struct for containing generated secrets to be used by the wyhash algorithm. Ensures it can't be used
/// incorrectly, and can only be constructed by [`super::WyHashLegacy::make_secret`].
pub struct LegacySecret([u64; 4]);

impl LegacySecret {
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
impl Debug for LegacySecret {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Secret").finish()
    }
}

/// Generate new secret for legacy final v4 wyhash. Takes a seed value and outputs 4 suitable `u64` constants
/// for use with the hasher. The PRNG will always use the default constants provided.
pub(super) const fn make_secret_legacy(mut seed: u64) -> LegacySecret {
    let mut secret: [u64; 4] = [0; 4];
    let mut i: usize = 0;

    while i < secret.len() {
        let mut ok: bool = false;

        while !ok {
            ok = true;
            secret[i] = 0;
            let mut j: usize = 0;

            while j < 64 {
                // WyRand... but const!
                let (value, new_state) = WyRandLegacy::gen_u64(seed);
                seed = new_state;
                let random_index = (value as usize) % C_VALUES.len();
                secret[i] |= (C_VALUES[random_index] as u64) << j;
                j += 8;
            }

            if secret[i] % 2 == 0 {
                ok = false;
                continue;
            }

            let mut j: usize = 0;

            while j < i {
                if (secret[j] ^ secret[i]).count_ones() != 32 {
                    ok = false;
                    break;
                }
                j += 1;
            }
        }

        i += 1;
    }

    LegacySecret(secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_expected_secrets() {
        let test_cases: [u64; 4] = [0, 3, 6, 42];
        let expected_results: [LegacySecret; 4] = [
            LegacySecret::new(
                0x95d49a959ca5a395,
                0xb4a9716ac94da695,
                0x5635cc6355956559,
                0xe1e18e3a9c591da9,
            ),
            LegacySecret::new(
                0xa9c64d71a6e2a3c9,
                0x5cac27591d9ad1e1,
                0x3574d14eb45987a5,
                0xd8b85963273c4d1d,
            ),
            LegacySecret::new(
                0x4dc3d12e36b1272d,
                0xaa5a8b35b4781d1b,
                0xcc36354be4e24e4b,
                0x3c554da34d748787,
            ),
            LegacySecret::new(
                0x4d781d729a998b95,
                0xa52e8ec66a3c5655,
                0xb4e89c6536272da3,
                0x6aacaaac8ee2c393,
            ),
        ];

        test_cases
            .into_iter()
            .zip(expected_results)
            .for_each(|(seed, expected)| {
                let result = make_secret_legacy(seed);

                assert_eq!(&result, &expected, "Failed secret for seed: {}", seed);
            });
    }
}

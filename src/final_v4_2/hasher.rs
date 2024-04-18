use core::hash::Hasher;

#[cfg(feature = "debug")]
use core::fmt::Debug;

use crate::{
    read::{read_4_bytes, read_8_bytes, read_upto_3_bytes},
    utils::{wymix, wymul},
};

use super::{
    constants::{WY0, WY1, WY2, WY3},
    secret::{make_secret, Secret},
};

/// The WyHash hasher, a fast & portable hashing algorithm. This implementation is
/// based on the final v4.2 C reference implementation.
///
/// ```
/// use wyrand::WyHash;
/// use core::hash::Hasher;
///
/// let mut hasher = WyHash::default();
///
/// hasher.write_u64(5);
///
/// assert_ne!(hasher.finish(), 5); // Should not be represented by the same value any more
/// ```
///
/// # Stability
///
/// The result is only guaranteed to match the result `wyhash` would naturally produce if `write`
/// is called a single time, followed by a call to `finish`.
///
/// Any other sequence of events (including calls to `write_u32` or similar functions) are
/// guaranteed to have consistent results between platforms and versions of this crate, but may not
/// map well to the reference implementation.
#[derive(Clone)]
pub struct WyHash {
    seed: u64,
    lo: u64,
    hi: u64,
    size: u64,
    secret: Secret,
}

impl WyHash {
    /// Create suitable secret values to be used by the hasher.
    #[must_use]
    #[inline(always)]
    pub fn make_secret(seed: u64) -> Secret {
        make_secret(seed)
    }

    /// Create hasher with seeds for the state and secret (generates a new secret, expensive to compute).
    pub const fn new(seed: u64, secret_seed: u64) -> Self {
        Self::new_with_secret(seed, make_secret(secret_seed))
    }

    /// Create hasher with a seed and default secrets
    #[inline]
    pub const fn new_with_default_secret(seed: u64) -> Self {
        Self::new_with_secret(seed, Secret::new(WY0, WY1, WY2, WY3))
    }

    /// Create hasher with a seed value and a secret. Assumes the user created the secret with [`make_secret`],
    /// else the hashing output will be weak/vulnerable.
    #[inline]
    pub const fn new_with_secret(mut seed: u64, secret: Secret) -> Self {
        seed ^= wymix(seed ^ secret.first(), secret.second());

        WyHash {
            seed,
            lo: 0,
            hi: 0,
            size: 0,
            secret,
        }
    }

    #[inline]
    fn consume_bytes(&self, bytes: &[u8]) -> (u64, u64, u64) {
        let length = bytes.len();
        if length == 0 {
            (0, 0, self.seed)
        } else if length <= 3 {
            (read_upto_3_bytes(bytes), 0, self.seed)
        } else if length <= 16 {
            let lo = (read_4_bytes(bytes) << 32) | read_4_bytes(&bytes[(length >> 3) << 2..]);
            let hi = (read_4_bytes(&bytes[length - 4..]) << 32)
                | read_4_bytes(&bytes[length - 4 - ((length >> 3) << 2)..]);
            (lo, hi, self.seed)
        } else {
            let mut index = length;
            let mut start = 0;
            let mut seed = self.seed;

            if length >= 48 {
                let mut seed1 = seed;
                let mut seed2 = seed;

                while index >= 48 {
                    seed = wymix(
                        read_8_bytes(&bytes[start..]) ^ self.secret.second(),
                        read_8_bytes(&bytes[start + 8..]) ^ seed,
                    );
                    seed1 = wymix(
                        read_8_bytes(&bytes[start + 16..]) ^ self.secret.third(),
                        read_8_bytes(&bytes[start + 24..]) ^ seed1,
                    );
                    seed2 = wymix(
                        read_8_bytes(&bytes[start + 32..]) ^ self.secret.fourth(),
                        read_8_bytes(&bytes[start + 40..]) ^ seed2,
                    );
                    index -= 48;
                    start += 48;
                }

                seed ^= seed1 ^ seed2;
            }

            while index > 16 {
                seed = wymix(
                    read_8_bytes(&bytes[start..]) ^ self.secret.second(),
                    read_8_bytes(&bytes[start + 8..]) ^ seed,
                );
                index -= 16;
                start += 16
            }

            let lo = read_8_bytes(&bytes[length - 16..]);
            let hi = read_8_bytes(&bytes[length - 8..]);
            (lo, hi, seed)
        }
    }

    #[inline]
    fn mix_current_seed(&mut self) {
        if self.size != 0 {
            self.seed = wymix(self.lo, self.hi ^ self.seed);
        }
    }
}

impl Hasher for WyHash {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.mix_current_seed();

        let (lo, hi, seed) = self.consume_bytes(bytes);

        self.lo = lo;
        self.hi = hi;
        self.seed = seed;
        self.size += bytes.len() as u64;
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.write_u64(i as u64)
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.write_u64(i as u64)
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.write_u64(i as u64)
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.mix_current_seed();
        self.lo = i;
        self.hi = 0;
        self.size += 8;
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.mix_current_seed();
        self.lo = i as u64;
        self.hi = (i >> 64) as u64;
        self.size += 16;
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn finish(&self) -> u64 {
        let (lo, hi) = wymul(self.lo ^ self.secret.second(), self.hi ^ self.seed);
        wymix(
            lo ^ self.secret.first() ^ self.size,
            hi ^ self.secret.second(),
        )
    }
}

impl Default for WyHash {
    #[inline]
    fn default() -> Self {
        WyHash::new_with_default_secret(0)
    }
}

#[cfg(feature = "debug")]
impl Debug for WyHash {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Do not expose the internal state of the Hasher
        f.debug_struct("WyHash")
            .field("size", &self.size)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    use core::hash::Hash;

    #[cfg(feature = "debug")]
    #[test]
    fn no_leaking_debug() {
        use alloc::format;

        let rng = WyHash::default();

        assert_eq!(
            format!("{rng:?}"),
            "WyHash { size: 0, .. }",
            "Debug should not be leaking sensitive internal state"
        );
    }

    #[rustfmt::skip]
    const TEST_VECTORS: [(u64, &str); 8] = [
        (0x9322_8a4d_e0ee_c5a2, ""),
        (0xc5ba_c3db_1787_13c4, "a"),
        (0xa97f_2f7b_1d9b_3314, "abc"),
        (0x786d_1f1d_f380_1df4, "message digest"),
        (0xdca5_a813_8ad3_7c87, "abcdefghijklmnopqrstuvwxyz"),
        (0xb9e7_34f1_17cf_af70, "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"),
        (0x6cc5_eab4_9a92_d617, "12345678901234567890123456789012345678901234567890123456789012345678901234567890"),
        (0xe1d4_c58d_97ba_df5e, "123456789012345678901234567890123456789012345678"),
    ];

    #[test]
    fn expected_hasher_output() {
        TEST_VECTORS
            .into_iter()
            .enumerate()
            .map(|(seed, (expected, input))| {
                let mut hasher = WyHash::new_with_default_secret(seed as u64);

                hasher.write(input.as_bytes());

                (input, expected, hasher.finish())
            })
            .for_each(|(input, expected_hash, computed_hash)| {
                assert_eq!(
                    expected_hash, computed_hash,
                    "Hashed output didn't match expected for \"{}\"",
                    input
                );
            });
    }

    #[test]
    fn multiple_writes_no_collision() {
        let mut hasher = WyHash::new_with_default_secret(0);
        hasher.write(b"abcdef");
        hasher.write(b"abcdef");
        let hash_a = hasher.finish();

        let mut hasher = WyHash::new_with_default_secret(0);
        hasher.write(b"abcdeF");
        hasher.write(b"abcdef");
        let hash_b = hasher.finish();

        assert_ne!(hash_a, hash_b);
    }

    #[test]
    fn tuples_no_collision() {
        let mut hasher = WyHash::new_with_default_secret(0);
        (1000, 2000).hash(&mut hasher);
        let hash_a = hasher.finish();

        let mut hasher = WyHash::new_with_default_secret(0);
        (1500, 2000).hash(&mut hasher);
        let hash_b = hasher.finish();

        assert_ne!(hash_a, hash_b);
    }
}

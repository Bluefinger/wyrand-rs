mod read;
mod secret;

use core::hash::Hasher;

#[cfg(feature = "debug")]
use core::fmt::Debug;

use crate::{
    constants::{WY0, WY1, WY2, WY3},
    utils::{wymix, wymul},
};

use self::{
    read::{wyread32, wyread64, wyread_upto_24},
    secret::make_secret,
};

/// WyHash hasher, a fast & portable hashing algorithm. This implementation is
/// based on the final v4 C reference implementation, as that is compatible with
/// the constants used for the current `WyRand` implementation.
#[derive(Clone)]
pub struct WyHash {
    seed: u64,
    first: u64,
    second: u64,
    size: u64,
    secret: [u64; 4],
}

impl WyHash {
    /// Create hasher with a seed and a secret
    pub fn new(seed: u64, secret_seed: u64) -> Self {
        Self::new_with_secret(seed, make_secret(secret_seed))
    }

    /// Create hasher with a seed and the default secrets
    #[inline]
    pub fn new_with_default_secret(seed: u64) -> Self {
        Self::new_with_secret(seed, [WY0, WY1, WY2, WY3])
    }

    #[inline]
    fn new_with_secret(mut seed: u64, secret: [u64; 4]) -> Self {
        seed ^= wymix(seed ^ secret[0], secret[1]);

        WyHash {
            seed,
            first: 0,
            second: 0,
            size: 0,
            secret,
        }
    }

    #[inline]
    fn consume_bytes(&mut self, bytes: &[u8]) {
        let (first, second): (u64, u64);
        let length = bytes.len();
        let mut seed = self.seed;

        match length {
            4..=16 => {
                first = (wyread32(bytes) << 32) | wyread32(&bytes[((length >> 3) << 2)..]);
                second = (wyread32(&bytes[(length - 4)..]) << 32)
                    | wyread32(&bytes[(length - 4 - ((length >> 3) << 2))..]);
            }
            1..=3 => {
                first = wyread_upto_24(bytes);
                second = 0;
            }
            0 => {
                first = 0;
                second = 0;
            }
            _ => {
                let mut index = length;
                let mut start = 0;
                if length > 48 {
                    let mut seed1 = seed;
                    let mut seed2 = seed;
                    while index > 48 {
                        seed = wymix(
                            wyread64(&bytes[start..]) ^ self.secret[1],
                            wyread64(&bytes[start + 8..]) ^ seed,
                        );
                        seed1 = wymix(
                            wyread64(&bytes[start + 16..]) ^ self.secret[2],
                            wyread64(&bytes[start + 24..]) ^ seed1,
                        );
                        seed2 = wymix(
                            wyread64(&bytes[start + 32..]) ^ self.secret[3],
                            wyread64(&bytes[start + 40..]) ^ seed2,
                        );
                        index -= 48;
                        start += 48;
                    }
                    seed ^= seed1 ^ seed2;
                }

                while index > 16 {
                    seed = wymix(
                        wyread64(&bytes[start..]) ^ self.secret[1],
                        wyread64(&bytes[start + 8..]) ^ seed,
                    );
                    index -= 16;
                    start += 16
                }

                first = wyread64(&bytes[(length - 16)..]);
                second = wyread64(&bytes[(length - 8)..]);
            }
        }

        self.first = first;
        self.second = second;
        self.seed = seed;
        self.size += length as u64;
    }
}

impl Hasher for WyHash {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for chunk in bytes.chunks(u64::MAX as usize) {
            self.consume_bytes(chunk);
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        let (first, second) = wymul(self.first ^ self.secret[1], self.second ^ self.seed);
        wymix(first ^ self.secret[0] ^ self.size, second ^ self.secret[1])
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
        f.debug_tuple("WyHash").finish()
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

        let rng = WyHash::default();

        assert_eq!(
            format!("{rng:?}"),
            "WyHash",
            "Debug should not be leaking internal state"
        );
    }

    #[test]
    fn expected_v4_hasher_output() {
        // Test cases generated from the C reference's test_vectors
        #[rustfmt::skip]
        let test_cases: [(u64, &str); 8] = [
            (0x0409_638e_e2bd_e459, ""),
            (0xa841_2d09_1b5f_e0a9, "a"),
            (0x32dd_92e4_b291_5153, "abc"),
            (0x8619_1240_89a3_a16b, "message digest"),
            (0x7a43_afb6_1d7f_5f40, "abcdefghijklmnopqrstuvwxyz"),
            (0xff42_329b_90e5_0d58, "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"),
            (0xc39c_ab13_b115_aad3, "12345678901234567890123456789012345678901234567890123456789012345678901234567890"),
            (0x21dc_4142_8fc5_4701, "1234567890"),
        ];

        test_cases
            .into_iter()
            .enumerate()
            .map(|(seed, (expected, input))| {
                let mut hasher = WyHash::new_with_secret(seed as u64, [WY0, WY1, WY2, WY3]);

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
}

use crate::WyRand;

#[cfg(feature = "v4_2")]
use crate::hasher::primes::is_prime;

/// Generate new secret for wyhash
pub(super) const fn make_secret(mut seed: u64) -> [u64; 4] {
    const C_VALUES: &[u8] = &[
        15, 23, 27, 29, 30, 39, 43, 45, 46, 51, 53, 54, 57, 58, 60, 71, 75, 77, 78, 83, 85, 86, 89,
        90, 92, 99, 101, 102, 105, 106, 108, 113, 114, 116, 120, 135, 139, 141, 142, 147, 149, 150,
        153, 154, 156, 163, 165, 166, 169, 170, 172, 177, 178, 180, 184, 195, 197, 198, 201, 202,
        204, 209, 210, 212, 216, 225, 226, 228, 232, 240,
    ];

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
                let (value, new_state) = WyRand::gen_u64(seed);
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

            #[cfg(feature = "v4_2")]
            if ok && !is_prime(secret[i]) {
                ok = false;
            }
        }

        i += 1;
    }

    secret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_expected_secrets() {
        let test_cases: [u64; 4] = [0, 3, 6, 42];
        #[cfg(feature = "v4_2")]
        let expected_results: [[u64; 4]; 4] = [
            [
                0x39d43c5c4e3a724b,
                0x6596e14753cca38b,
                0xc68d954b2b339353,
                0x96b4a6e45c65aa55,
            ],
            [
                0xa3743ca35956ac59,
                0x65b1b8e8558b72c5,
                0x78cad4b8c98ea535,
                0x561d59965a4baa27,
            ],
            [
                0x993c394d599a9a2b,
                0x535c4d3c9ae1a91d,
                0x72b2356a3cc6f0a5,
                0x5a6c8e1b6c2e4da9,
            ],
            [
                0x8b4be21b934dc6a3,
                0x9a0f72f0e81b6969,
                0x99746a47f066331b,
                0xccb8b85a99aaa9b1,
            ],
        ];
        #[cfg(not(feature = "v4_2"))]
        let expected_results: [[u64; 4]; 4] = [
            [
                0x95d49a959ca5a395,
                0xb4a9716ac94da695,
                0x5635cc6355956559,
                0xe1e18e3a9c591da9,
            ],
            [
                0xa9c64d71a6e2a3c9,
                0x5cac27591d9ad1e1,
                0x3574d14eb45987a5,
                0xd8b85963273c4d1d,
            ],
            [
                0x4dc3d12e36b1272d,
                0xaa5a8b35b4781d1b,
                0xcc36354be4e24e4b,
                0x3c554da34d748787,
            ],
            [
                0x4d781d729a998b95,
                0xa52e8ec66a3c5655,
                0xb4e89c6536272da3,
                0x6aacaaac8ee2c393,
            ],
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

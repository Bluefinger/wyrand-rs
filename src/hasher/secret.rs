use crate::WyRand;

/// Generate new secret for wyhash
pub(super) fn make_secret(seed: u64) -> [u64; 4] {
    const C_VALUES: &[u8] = &[
        15_u8, 23, 27, 29, 30, 39, 43, 45, 46, 51, 53, 54, 57, 58, 60, 71, 75, 77, 78, 83, 85, 86,
        89, 90, 92, 99, 101, 102, 105, 106, 108, 113, 114, 116, 120, 135, 139, 141, 142, 147, 149,
        150, 153, 154, 156, 163, 165, 166, 169, 170, 172, 177, 178, 180, 184, 195, 197, 198, 201,
        202, 204, 209, 210, 212, 216, 225, 226, 228, 232, 240,
    ];

    let mut secret: [u64; 4] = [0; 4];
    let mut rng = WyRand::new(seed);

    for i in 0..secret.len() {
        loop {
            secret[i] = 0;
            for j in (0..64).step_by(8) {
                let value = rng.rand() as usize;
                secret[i] |= u64::from(C_VALUES[value % C_VALUES.len()]) << j;
            }
            if secret[i] % 2 == 0 {
                continue;
            }
            let incorrect_number_of_ones_found = (0..i)
                .step_by(1)
                .find(|&j| (secret[j] ^ secret[i]).count_ones() != 32);
            if incorrect_number_of_ones_found.is_none() {
                break;
            }
        }
    }
    secret
}

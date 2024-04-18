/// Expand the state to 128 bits and multiply with the expanded second value,
/// before xor'ing the hi & lo result back to 64 bits.
#[inline(always)]
pub(crate) const fn wymix(first: u64, second: u64) -> u64 {
    let (lo, hi) = wymul(first, second);
    lo ^ hi
}

/// Expand and multiply to 128 bits, then return the hi and lo components
/// of the multiplication as u64 values.
#[inline(always)]
pub(crate) const fn wymul(first: u64, second: u64) -> (u64, u64) {
    let total = (first as u128).wrapping_mul(second as u128);
    (total as u64, total.wrapping_shr(64) as u64)
}

#[cfg(feature = "wyhash")]
#[inline(always)]
pub(crate) const fn check_for_valid_secret_value(current_value: usize, secret: &[u64; 4]) -> bool {
    if secret[current_value] % 2 == 0 {
        return false;
    }

    let mut prev_value: usize = 0;

    while prev_value < current_value {
        if (secret[prev_value] ^ secret[current_value]).count_ones() != 32 {
            return false;
        }
        prev_value += 1;
    }

    true
}

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

#[cfg(feature = "randomised_wyhash")]
#[inline]
pub(crate) fn get_random_u64() -> u64 {
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
        use rand_core::RngCore;

        // This is faster than doing `.fill_bytes()`. User-space entropy goes brrr.
        rand::thread_rng().next_u64()
    }
}

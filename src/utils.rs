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

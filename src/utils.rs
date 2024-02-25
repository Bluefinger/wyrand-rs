/// Expand the state to 128 bits and multiply with the expanded second value,
/// before xor'ing the hi & lo result back to 64 bits.
#[inline(always)]
pub(crate) fn wymix(first: u64, second: u64) -> u64 {
    let (a, b) = wymul(first, second);
    a ^ b
}

/// Expand and multiply to 128 bits, then return the hi and lo components
/// of the multiplication as u64 values.
#[inline(always)]
pub(crate) fn wymul(first: u64, second: u64) -> (u64, u64) {
    let t = u128::from(first).wrapping_mul(u128::from(second));
    (t as u64, t.wrapping_shr(64) as u64)
}

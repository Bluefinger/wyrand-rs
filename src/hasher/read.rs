#[inline(always)]
pub(super) const fn wyread64(bits: &[u8]) -> u64 {
    (bits[7] as u64) << 56
        | (bits[6] as u64) << 48
        | (bits[5] as u64) << 40
        | (bits[4] as u64) << 32
        | (bits[3] as u64) << 24
        | (bits[2] as u64) << 16
        | (bits[1] as u64) << 8
        | (bits[0] as u64)
}

#[inline(always)]
pub(super) const fn wyread32(bits: &[u8]) -> u64 {
    (bits[3] as u64) << 24
        | (bits[2] as u64) << 16
        | (bits[1] as u64) << 8
        | (bits[0] as u64)
}

#[inline(always)]
pub(super) const fn wyread_upto_24(bits: &[u8]) -> u64 {
    (bits[0] as u64) << 16
        | (bits[bits.len() >> 1] as u64) << 8
        | (bits[bits.len() - 1] as u64)
}

#[inline(always)]
pub(super) const fn is_over_48_bytes(length: usize) -> bool {
    #[cfg(feature = "v4_2")]
    {
        length >= 48
    }
    #[cfg(not(feature = "v4_2"))]
    {
        length > 48
    }
}

#[inline(always)]
pub(super) fn wyread64(bits: &[u8]) -> u64 {
    u64::from(bits[7]) << 56
        | u64::from(bits[6]) << 48
        | u64::from(bits[5]) << 40
        | u64::from(bits[4]) << 32
        | u64::from(bits[3]) << 24
        | u64::from(bits[2]) << 16
        | u64::from(bits[1]) << 8
        | u64::from(bits[0])
}

#[inline(always)]
pub(super) fn wyread32(bits: &[u8]) -> u64 {
    u64::from(bits[3]) << 24
        | u64::from(bits[2]) << 16
        | u64::from(bits[1]) << 8
        | u64::from(bits[0])
}

#[inline(always)]
pub(super) fn wyread_upto_24(bits: &[u8]) -> u64 {
    u64::from(bits[0]) << 16
        | u64::from(bits[bits.len() >> 1]) << 8
        | u64::from(bits[bits.len() - 1])
}

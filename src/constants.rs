#[cfg(not(feature = "v4_2"))]
mod v4 {
    pub(crate) const WY0: u64 = 0xa076_1d64_78bd_642f;
    pub(crate) const WY1: u64 = 0xe703_7ed1_a0b4_28db;
    #[cfg(feature = "wyhash")]
    pub(crate) const WY2: u64 = 0x8ebc_6af0_9c88_c6e3;
    #[cfg(feature = "wyhash")]
    pub(crate) const WY3: u64 = 0x5899_65cc_7537_4cc3;
}

#[cfg(feature = "v4_2")]
mod v4_2 {
    pub(crate) const WY0: u64 = 0x2d35_8dcc_aa6c_78a5;
    pub(crate) const WY1: u64 = 0x8bb8_4b93_962e_acc9;
    #[cfg(feature = "wyhash")]
    pub(crate) const WY2: u64 = 0x4b33_a62e_d433_d4a3;
    #[cfg(feature = "wyhash")]
    pub(crate) const WY3: u64 = 0x4d5a_2da5_1de1_aa47;
}

#[cfg(not(feature = "v4_2"))]
pub(crate) use v4::*;

#[cfg(feature = "v4_2")]
pub(crate) use v4_2::*;

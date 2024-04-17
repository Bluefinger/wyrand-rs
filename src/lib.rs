#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![no_std]
#![doc = include_str!("../README.md")]

#[cfg(feature = "fully_randomised_wyhash")]
extern crate std;

mod utils;
#[cfg(feature = "wyhash")]
mod read;
/// Legacy final v4 implementations of `WyRand` & `WyHash`. These are the older legacy version of the algorithms,
/// only use them if for whatever reason you favour stability with previous usages over to switching to
/// more secure versions of the algorithm.
#[cfg(feature = "legacy_v4")]
#[cfg_attr(docsrs, doc(cfg(feature = "legacy_v4")))]
pub mod legacy_final_v4;
mod final_v4_2;
#[cfg(feature = "wyhash")]
mod constants;

pub use final_v4_2::*;

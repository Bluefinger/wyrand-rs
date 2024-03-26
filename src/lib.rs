#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![no_std]
#![doc = include_str!("../README.md")]

mod constants;
#[cfg(feature = "wyhash")]
mod hasher;
mod wyrand;
mod utils;

#[cfg(feature = "wyhash")]
pub use hasher::*;
pub use wyrand::WyRand;

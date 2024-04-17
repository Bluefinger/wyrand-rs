#[cfg(feature = "randomised_wyhash")]
mod builder;
mod constants;
#[cfg(feature = "wyhash")]
mod hasher;
#[cfg(feature = "wyhash")]
mod secret;
mod wyrand;

#[cfg(feature = "randomised_wyhash")]
#[cfg_attr(docsrs, doc(cfg(feature = "randomised_wyhash")))]
pub use builder::RandomWyHashLegacyState;

#[cfg(feature = "wyhash")]
#[cfg_attr(docsrs, doc(cfg(feature = "wyhash")))]
pub use hasher::WyHashLegacy;

#[cfg(feature = "wyhash")]
#[cfg_attr(docsrs, doc(cfg(feature = "wyhash")))]
pub use secret::LegacySecret;

pub use wyrand::WyRandLegacy;

#[cfg(feature = "randomised_wyhash")]
mod builder;
mod constants;
#[cfg(feature = "wyhash")]
mod hasher;
#[cfg(feature = "wyhash")]
mod primes;
#[cfg(feature = "wyhash")]
mod secret;
mod wyrand;

#[cfg(feature = "randomised_wyhash")]
#[cfg_attr(docsrs, doc(cfg(feature = "randomised_wyhash")))]
pub use builder::RandomWyHashState;

#[cfg(feature = "wyhash")]
#[cfg_attr(docsrs, doc(cfg(feature = "wyhash")))]
pub use hasher::WyHash;

#[cfg(feature = "wyhash")]
#[cfg_attr(docsrs, doc(cfg(feature = "wyhash")))]
pub use secret::Secret;

pub use wyrand::WyRand;

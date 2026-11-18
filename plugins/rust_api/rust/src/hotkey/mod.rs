#[cfg(all(not(target_os = "android"), not(target_os = "ios")))]
mod keys;
#[cfg(all(not(target_os = "android"), not(target_os = "ios")))]
mod owner;
#[cfg(all(not(target_os = "android"), not(target_os = "ios")))]
mod service;
#[cfg(any(target_os = "android", target_os = "ios"))]
mod unsupported;

#[cfg(all(not(target_os = "android"), not(target_os = "ios")))]
pub use service::{listen, set_hot_keys};
#[cfg(any(target_os = "android", target_os = "ios"))]
pub use unsupported::{listen, set_hot_keys};

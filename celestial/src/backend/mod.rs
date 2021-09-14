
/// The backend trait for serializing and deserializing data
pub trait Backend {}

#[cfg(any(feature = "json", doc))]
#[doc(cfg(feature = "json"))]
pub mod json;
#[cfg(any(feature = "bincode", doc))]
#[doc(cfg(feature = "bincode"))]
pub mod bincode;
#[cfg(any(feature = "cbor", doc))]
#[doc(cfg(feature = "cbor"))]
pub mod cbor;
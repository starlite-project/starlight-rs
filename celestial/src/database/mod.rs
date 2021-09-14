use crate::backend::{bincode::BincodeBackend, cbor::CborBackend, json::JsonBackend, Backend};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Database<B: Backend> {
	_marker: PhantomData<B>,
}

#[cfg(any(feature = "json", doc))]
#[doc(cfg(feature = "json"))]
pub type JsonDatabase = Database<JsonBackend>;

#[cfg(any(feature = "bincode", doc))]
#[doc(cfg(feature = "bincode"))]
pub type BincodeDatabase = Database<BincodeBackend>;

#[cfg(any(feature = "cbor", doc))]
#[doc(cfg(feature = "cbor"))]
pub type CBorDatabase = Database<CborBackend>;

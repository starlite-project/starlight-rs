pub mod gateway;

use std::hash::Hash;

pub trait Entity: Send + Sync {
    type Id: Copy + Eq + Hash + Send + Sync;

    fn id(&self) -> Self::Id;
}

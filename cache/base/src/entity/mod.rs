pub mod channel;
pub mod gateway;
pub mod guild;
pub mod user;
pub mod voice;

use std::hash::Hash;

pub trait Entity: Send + Sync {
    type Id: Copy + Eq + Hash + Send + Sync;

    fn id(&self) -> Self::Id;
}

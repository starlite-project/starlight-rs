use nebula::Id;
use serde::{Deserialize, Serialize};
use starchart::Value;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GuildSettings {
	pub id: Id,
}

impl Value for GuildSettings {
	type Key = Id;

	fn key(&self) -> Self::Key {
		self.id
	}
}

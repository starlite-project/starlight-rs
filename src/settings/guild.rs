use serde::{Deserialize, Serialize};
use twilight_model::id::{
	marker::{GuildMarker, UserMarker},
	Id,
};
use starchart::IndexEntry;

#[derive(Debug, Clone, Serialize, Deserialize, IndexEntry)]
pub struct GuildSettings {
	id: Id<GuildMarker>,
	tags: Vec<GuildTag>,
}

impl GuildSettings {
	#[must_use]
	pub const fn new(id: Id<GuildMarker>) -> Self {
		Self { id, tags: Vec::new() }
	}
}

impl Default for GuildSettings {
	fn default() -> Self {
		Self::new(Id::new(1))
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildTag {
	name: String,
	description: String,
	author: Id<UserMarker>,
	aliases: Vec<String>,
}

impl GuildTag {
	#[must_use]
	pub const fn new(name: String, description: String, author: Id<UserMarker>) -> Self {
		Self {
			name,
			description,
			author,
			aliases: Vec::new(),
		}
	}
}

use serde::{Deserialize, Serialize};
use twilight_model::id::{marker::UserMarker, Id};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildTag {
	name: String,
	description: String,
	author: Id<UserMarker>,
}

impl GuildTag {
	#[must_use]
	pub const fn new(name: String, description: String, author: Id<UserMarker>) -> Self {
		Self {
			name,
			description,
			author,
		}
	}

	#[must_use]
	pub fn name(&self) -> &str {
		&self.name
	}

	#[must_use]
	pub fn description(&self) -> &str {
		&self.description
	}

	#[must_use]
	pub const fn author(&self) -> Id<UserMarker> {
		self.author
	}

	pub fn set_description(&mut self, content: String) {
		self.description = content;
	}
}

impl Default for GuildTag {
	fn default() -> Self {
		Self {
			name: String::new(),
			description: String::new(),
			author: unsafe { Id::new_unchecked(1) },
		}
	}
}

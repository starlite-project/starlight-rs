use std::iter::Extend;

use serde::{Deserialize, Serialize};
use starchart::IndexEntry;
use twilight_model::id::{
	marker::{GuildMarker, UserMarker},
	Id,
};

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Clone, IndexEntry, Serialize, Deserialize)]
pub struct GuildSettings {
	id: Id<GuildMarker>,
	tags: Vec<GuildTag>,
}

impl GuildSettings {
	#[must_use]
	pub const fn new(id: Id<GuildMarker>) -> Self {
		Self {
			id,
			tags: Vec::new(),
		}
	}

	#[must_use]
	pub const fn id(&self) -> Id<GuildMarker> {
		self.id
	}

	#[must_use]
	pub fn tags(&self) -> &[GuildTag] {
		&self.tags
	}

	pub fn push_tag(&mut self, tag: GuildTag) {
		self.tags.push(tag);
	}

	pub fn remove_tag(&mut self, tag_name: &str) -> Option<GuildTag> {
		let position = self.tags().iter().position(|x| x.name == tag_name)?;
		Some(self.tags.swap_remove(position))
	}

	#[must_use]
	pub fn tags_mut(&mut self) -> &mut [GuildTag] {
		&mut self.tags
	}
}

impl Default for GuildSettings {
	fn default() -> Self {
		let default_tags = vec![GuildTag::default()];

		Self {
			id: unsafe { Id::new_unchecked(1) },
			tags: default_tags,
		}
	}
}

impl Extend<GuildTag> for GuildSettings {
	fn extend<T: IntoIterator<Item = GuildTag>>(&mut self, iter: T) {
		for tag in iter {
			self.push_tag(tag);
		}
	}
}

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

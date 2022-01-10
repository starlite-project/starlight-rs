use std::{collections::HashMap, iter::Extend};

use serde::{
	Deserialize, Serialize,
};
use starchart::IndexEntry;
use twilight_model::id::{
	marker::{GuildMarker, UserMarker},
	Id,
};

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Clone, IndexEntry, Serialize, Deserialize)]
pub struct GuildSettings {
	id: Id<GuildMarker>,
	tags: HashMap<String, GuildTag>,
}

impl GuildSettings {
	#[must_use]
	pub fn new(id: Id<GuildMarker>) -> Self {
		Self {
			id,
			tags: HashMap::new(),
		}
	}

	pub fn insert_tag(&mut self, tag: GuildTag) -> Option<GuildTag> {
		self.tags.insert(tag.name.clone(), tag)
	}

	pub fn get_tag<Q: AsRef<str>>(&self, tag: &Q) -> Option<&GuildTag> {
		let tag = tag.as_ref();
		self.tags
			.get(tag)
			.or_else(|| self.tags.values().find(|t| t.name == tag))
	}
}

impl Default for GuildSettings {
	fn default() -> Self {
		let default_map = HashMap::from([("default".to_owned(), GuildTag::default())]);

		Self {
			id: unsafe { Id::new_unchecked(1) },
			tags: default_map,
		}
	}
}

impl Extend<GuildTag> for GuildSettings {
	fn extend<T: IntoIterator<Item = GuildTag>>(&mut self, iter: T) {
		for tag in iter {
			self.insert_tag(tag);
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

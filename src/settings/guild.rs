use std::{borrow::Borrow, collections::HashMap, hash::Hash, iter::Extend};

use serde::{Deserialize, Serialize};
use starchart::IndexEntry;
use twilight_model::id::{
	marker::{GuildMarker, UserMarker},
	Id,
};

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Clone, Serialize, Deserialize, IndexEntry)]
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

	pub fn get_tag<Q: AsRef<str>>(&self, tag: &Q) -> Option<&GuildTag>
	{
		let tag = tag.as_ref();
		self.tags
			.get(tag)
			.or_else(|| self.tags.values().find(|t| t.name == tag))
	}
}

impl Default for GuildSettings {
	fn default() -> Self {
		let default_map = HashMap::from([("".to_owned(), GuildTag::default())]);

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

	#[must_use]
	pub fn aliases(&self) -> &[String] {
		&self.aliases
	}

	pub fn aliases_mut(&mut self) -> &mut [String] {
		&mut self.aliases
	}

	pub fn push_alias(&mut self, alias: String) {
		self.aliases.push(alias);
	}
}

impl Default for GuildTag {
	fn default() -> Self {
		Self {
			name: String::new(),
			description: String::new(),
			author: unsafe { Id::new_unchecked(1) },
			aliases: vec![String::new()],
		}
	}
}

impl Extend<String> for GuildTag {
	fn extend<T: IntoIterator<Item = String>>(&mut self, iter: T) {
		self.aliases.extend(iter);
	}
}

use std::{collections::HashMap, iter::Extend};

use serde::{
	de::{Error as DeError, SeqAccess, Visitor},
	ser::SerializeStruct,
	Deserialize, Deserializer, Serialize, Serializer,
};
use starchart::IndexEntry;
use twilight_model::id::{
	marker::{GuildMarker, UserMarker},
	Id,
};

use crate::prelude::*;

const TAG_DE_ERROR_MESSAGE: &str = "a GuildTag with 4 elements";
const GS_DE_ERROR_MESSAGE: &str = "a GuildSettings with 2 elements";

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Clone, IndexEntry)]
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

impl Serialize for GuildSettings {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let tags = self.tags.values().cloned().collect::<Vec<_>>();
		let mut state = serializer.serialize_struct("GuildSettings", 2)?;
		state.serialize_field("id", &self.id.get())?;
		state.serialize_field("tags", &tags[..])?;
		state.end()
	}
}

struct GuildVisitor;

impl<'de> Visitor<'de> for GuildVisitor {
	type Value = GuildSettings;

	fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
		formatter.write_str("a GuildSettings")
	}

	fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
	where
		A: SeqAccess<'de>,
	{
		let id = unsafe {
			let snowflake = seq
				.next_element::<u64>()?
				.ok_or_else(|| DeError::invalid_length(0, &GS_DE_ERROR_MESSAGE))?;

			Id::new_unchecked(snowflake)
		};
		let tags = {
			let sequence_of_tags = seq
				.next_element::<Vec<GuildTag>>()?
				.ok_or_else(|| DeError::invalid_length(1, &GS_DE_ERROR_MESSAGE))?;

			sequence_of_tags
				.into_iter()
				.map(|tag| (tag.name.clone(), tag))
				.collect()
		};

		Ok(GuildSettings { id, tags })
	}
}

impl<'de> Deserialize<'de> for GuildSettings {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_seq(GuildVisitor)
	}
}

#[derive(Debug, Clone)]
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

impl Serialize for GuildTag {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let aliases = self.aliases.iter().map(String::as_str).collect::<Vec<_>>();
		let mut state = serializer.serialize_struct("GuildTag", 4)?;
		state.serialize_field("name", self.name.as_str())?;
		state.serialize_field("description", self.description.as_str())?;
		state.serialize_field("author", &self.author.get())?;
		state.serialize_field("aliases", &aliases[..])?;
		state.end()
	}
}

struct TagVisitor;

impl<'de> Visitor<'de> for TagVisitor {
	type Value = GuildTag;

	fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
		formatter.write_str("a GuildTag")
	}

	fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
	where
		A: SeqAccess<'de>,
	{
		let name = seq
			.next_element::<String>()?
			.ok_or_else(|| DeError::invalid_length(0, &TAG_DE_ERROR_MESSAGE))?;
		let description = seq
			.next_element::<String>()?
			.ok_or_else(|| DeError::invalid_length(1, &TAG_DE_ERROR_MESSAGE))?;
		let author = unsafe {
			let id = seq
				.next_element::<u64>()?
				.ok_or_else(|| DeError::invalid_length(2, &TAG_DE_ERROR_MESSAGE))?;
			Id::new_unchecked(id)
		};
		let aliases = seq
			.next_element::<Vec<String>>()?
			.ok_or_else(|| DeError::invalid_length(3, &TAG_DE_ERROR_MESSAGE))?;

		Ok(GuildTag {
			name,
			description,
			author,
			aliases,
		})
	}
}

impl<'de> Deserialize<'de> for GuildTag {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_seq(TagVisitor)
	}
}

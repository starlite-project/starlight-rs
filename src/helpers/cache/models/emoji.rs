use serde::{Deserialize, Serialize};
use twilight_cache_inmemory::model::CachedEmoji;
use twilight_model::{
	guild::Emoji,
	id::{EmojiId, RoleId},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmojiHelper {
	pub id: EmojiId,
	pub animated: bool,
	pub name: String,
	pub managed: bool,
	pub require_colons: bool,
	pub roles: Vec<RoleId>,
	pub available: bool,
}

impl PartialEq<Emoji> for EmojiHelper {
	fn eq(&self, other: &Emoji) -> bool {
		self.id == other.id
			&& self.animated == other.animated
			&& self.name == other.name
			&& self.require_colons == other.require_colons
			&& self.roles == other.roles
			&& self.available == other.available
	}
}

impl PartialEq<CachedEmoji> for EmojiHelper {
	fn eq(&self, other: &CachedEmoji) -> bool {
		self.id == other.id
			&& self.animated == other.animated
			&& self.name == other.name
			&& self.require_colons == other.require_colons
			&& self.roles == other.roles
			&& self.available == other.available
	}
}

impl From<Emoji> for EmojiHelper {
	fn from(emoji: Emoji) -> Self {
		Self {
			id: emoji.id,
			animated: emoji.animated,
			name: emoji.name,
			managed: emoji.managed,
			require_colons: emoji.require_colons,
			roles: emoji.roles,
			available: emoji.available,
		}
	}
}

impl From<CachedEmoji> for EmojiHelper {
	fn from(emoji: CachedEmoji) -> Self {
		Self {
			id: emoji.id,
			animated: emoji.animated,
			name: emoji.name,
			managed: emoji.managed,
			require_colons: emoji.require_colons,
			roles: emoji.roles,
			available: emoji.available,
		}
	}
}

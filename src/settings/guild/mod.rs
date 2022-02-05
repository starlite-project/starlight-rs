mod blocked_user;
mod tag;

use std::iter::Extend;

use serde::{Deserialize, Serialize};
use starchart::IndexEntry;
use twilight_model::id::{
	marker::{GuildMarker, UserMarker},
	Id,
};

pub use self::{blocked_user::BlockedUser, tag::GuildTag};

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Clone, IndexEntry, Serialize, Deserialize)]
pub struct GuildSettings {
	id: Id<GuildMarker>,
	tags: Vec<GuildTag>,
	blocked_users: Vec<BlockedUser>,
}

impl GuildSettings {
	#[must_use]
	pub const fn new(id: Id<GuildMarker>) -> Self {
		Self {
			id,
			tags: Vec::new(),
			blocked_users: Vec::new(),
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

	#[must_use]
	pub fn tags_mut(&mut self) -> &mut [GuildTag] {
		&mut self.tags
	}

	#[must_use]
	pub fn blocked_users(&self) -> &[BlockedUser] {
		&self.blocked_users
	}

	#[must_use]
	pub fn blocked_users_mut(&mut self) -> &mut [BlockedUser] {
		&mut self.blocked_users
	}

	pub fn add_tag(&mut self, tag: GuildTag) {
		self.tags.push(tag);
	}

	pub fn remove_tag(&mut self, tag_name: &str) -> Option<GuildTag> {
		let position = self.tags().iter().position(|x| x.name() == tag_name)?;
		Some(self.tags.swap_remove(position))
	}

	pub fn add_user(&mut self, user: BlockedUser) {
		self.blocked_users.push(user);
	}

	pub fn remove_user(&mut self, id: Id<UserMarker>) -> Option<BlockedUser> {
		let position = self.blocked_users().iter().position(|x| x.id() == id)?;
		Some(self.blocked_users.swap_remove(position))
	}
}

impl Default for GuildSettings {
	fn default() -> Self {
		let default_tags = vec![GuildTag::default()];
		let default_users = vec![BlockedUser::default()];

		Self {
			id: unsafe { Id::new_unchecked(1) },
			tags: default_tags,
			blocked_users: default_users,
		}
	}
}

impl Extend<GuildTag> for GuildSettings {
	fn extend<T: IntoIterator<Item = GuildTag>>(&mut self, iter: T) {
		self.tags.extend(iter);
	}
}

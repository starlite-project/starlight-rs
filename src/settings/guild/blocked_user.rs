use serde::{Deserialize, Serialize};
use twilight_model::id::{marker::UserMarker, Id};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockedUser {
	id: Id<UserMarker>,
	reason: String,
}

impl BlockedUser {
	#[must_use]
	pub const fn new(id: Id<UserMarker>, reason: String) -> Self {
		Self { id, reason }
	}

	#[must_use]
	pub const fn id(&self) -> Id<UserMarker> {
		self.id
	}

	#[must_use]
	pub fn reason(&self) -> &str {
		&self.reason
	}
}

impl Default for BlockedUser {
	fn default() -> Self {
		Self::new(unsafe { Id::new_unchecked(1) }, "".to_owned())
	}
}

use starchart::IndexEntry;
use twilight_model::id::{
	marker::{ApplicationMarker, UserMarker},
	Id,
};

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, IndexEntry)]
pub struct GlobalSettings {
	id: Id<ApplicationMarker>,
	owners: Vec<Id<UserMarker>>,
}

impl GlobalSettings {
	pub fn new(id: Id<ApplicationMarker>, owners: Vec<Id<UserMarker>>) -> Self {
		Self {
			id,
			owners,
		}
	}

	pub const fn id(&self) -> Id<ApplicationMarker> {
		self.id
	}

	pub fn owners(&self) -> &[Id<UserMarker>] {
		&self.owners
	}
}

impl Default for GlobalSettings {
	fn default() -> Self {
		let default_owners = unsafe { vec![Id::new_unchecked(1)] };

		Self {
			id: unsafe { Id::new_unchecked(1) },
			owners: default_owners,
		}
	}
}

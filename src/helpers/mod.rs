use crate::state::Context;

mod interactions;
mod color;

pub use self::{interactions::InteractionsHelper, color::Color};

pub const STARLIGHT_COLORS: [Color; 3] = [
	Color::new(132, 61, 164),
	Color::new(218, 0, 78),
	Color::new(183, 47, 0)
];

#[derive(Debug, Clone, Copy)]
#[must_use = "Helpers do nothing if not used"]
pub struct Helpers {
	context: Context,
}

impl Helpers {
	pub const fn new(context: Context) -> Self {
		Self { context }
	}

	pub const fn interactions(self) -> InteractionsHelper {
		InteractionsHelper::new(self)
	}

	#[must_use = "getting the current Context has no side effects"]
	pub const fn context(self) -> Context {
		self.context
	}
}

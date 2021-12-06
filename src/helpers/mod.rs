use crate::state::{Context, QuickAccess};

mod color;
mod interactions;
pub mod parsing;
pub mod playground;

pub use self::{color::Color, interactions::InteractionsHelper};

pub const STARLIGHT_COLORS: [Color; 3] = [
	Color::new(132, 61, 164),
	Color::new(218, 0, 78),
	Color::new(183, 47, 0),
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
}

impl QuickAccess for Helpers {
	fn context(&self) -> Context {
		self.context
	}
}

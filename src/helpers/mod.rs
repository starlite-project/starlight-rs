use crate::state::Context;

mod interactions;

pub use self::interactions::InteractionsHelper;

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

use thiserror::Error;
use twilight_model::application::component::Component;

pub trait ComponentBuilder {
	type Target;

	fn build(self) -> Result<Self::Target, BuildError>;

	fn build_component(self) -> Result<Component, BuildError>;
}

#[derive(Debug, Error, Clone, Copy)]
#[error("an error occurred while building the component")]
pub struct BuildError;

#[cfg(test)]
mod tests {
	use super::{BuildError, ComponentBuilder};
	use static_assertions::{assert_impl_all, assert_obj_safe};
	use std::{error::Error, fmt::Debug};
	use twilight_model::application::component::Component;

	assert_obj_safe!(ComponentBuilder<Target = ()>);

	assert_impl_all!(BuildError: Debug, Error, Send, Sync);

	#[test]
	#[should_panic]
	fn builder_fail() {
		#[derive(Clone, Copy)]
		struct Fail;

		impl ComponentBuilder for Fail {
			type Target = ();

			fn build(self) -> Result<Self::Target, BuildError> {
				Err(BuildError)
			}

			fn build_component(self) -> Result<Component, BuildError> {
				Err(BuildError)
			}
		}

		let value = Fail;

		value.build().unwrap();
	}

	#[test]
	fn builder_pass() -> Result<(), BuildError> {
		struct Builder;

		impl ComponentBuilder for Builder {
			type Target = u64;

			fn build(self) -> Result<Self::Target, BuildError> {
				Ok(10)
			}

			fn build_component(self) -> Result<Component, BuildError> {
				Err(BuildError)
			}
		}

		let builder = Builder;

		assert_eq!(builder.build()?, 10);

		Ok(())
	}
}

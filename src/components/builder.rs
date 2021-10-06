use thiserror::Error;
use twilight_model::application::component::Component;

pub trait ComponentBuilder {
	type Target;

	fn build(self) -> Result<Self::Target, BuildError>;

	fn build_component(self) -> Result<Component, BuildError>;
}

#[derive(Debug, Error, Clone, Copy)]
pub enum BuildError {
	#[error("the value `{0}` was not set")]
	ValueNotSet(&'static str),
	#[error("an invalid component type was passed")]
	InvalidComponentType,
	#[error("unable to build into a component")]
	NotBuildable,
	#[cfg(test)]
	#[cfg_attr(test, doc(hidden))]
	#[cfg_attr(test, error("this is a testing error"))]
	Testing,
}

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
				Err(BuildError::Testing)
			}

			fn build_component(self) -> Result<Component, BuildError> {
				Err(BuildError::Testing)
			}
		}

		let value = Fail;

		value.build().unwrap();
	}

	#[test]
	#[allow(clippy::panic_in_result_fn)]
	fn builder_pass() -> Result<(), BuildError> {
		struct Builder;

		impl ComponentBuilder for Builder {
			type Target = u64;

			fn build(self) -> Result<Self::Target, BuildError> {
				Ok(10)
			}

			fn build_component(self) -> Result<Component, BuildError> {
				Err(BuildError::Testing)
			}
		}

		let builder = Builder;

		assert_eq!(builder.build()?, 10);

		Ok(())
	}
}

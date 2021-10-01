#![allow(dead_code)]

use super::{BuildError, ComponentBuilder};
use twilight_model::{
	application::component::{select_menu::SelectMenuOption, Component, SelectMenu},
	channel::ReactionType,
};

#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct SelectMenuBuilder {
	pub custom_id: Option<String>,
	pub disabled: Option<bool>,
	pub max_values: Option<u8>,
	pub min_values: Option<u8>,
	pub options: Vec<SelectMenuOptionBuilder>,
	pub placeholder: Option<String>,
}

impl SelectMenuBuilder {
	#[must_use]
	pub const fn new() -> Self {
		Self {
			custom_id: None,
			disabled: None,
			max_values: None,
			min_values: None,
			options: vec![],
			placeholder: None,
		}
	}

	pub fn custom_id(&mut self, id: impl Into<String>) -> &mut Self {
		self.custom_id = Some(id.into());

		self
	}

	pub fn set_disabled(&mut self, value: bool) -> &mut Self {
		self.disabled = Some(value);

		self
	}

	pub fn max_values(&mut self, values: u8) -> &mut Self {
		self.max_values = Some(values);

		self
	}

	pub fn min_values(&mut self, values: u8) -> &mut Self {
		self.min_values = Some(values);

		self
	}

	pub fn option<F>(&mut self, option_fn: F) -> &mut Self
	where
		F: FnOnce(SelectMenuOptionBuilder) -> SelectMenuOptionBuilder,
	{
		let option = option_fn(SelectMenuOptionBuilder::default());

		self.options.push(option);

		self
	}

	pub fn placeholder(&mut self, value: impl Into<String>) -> &mut Self {
		self.placeholder = Some(value.into());

		self
	}
}

impl ComponentBuilder for SelectMenuBuilder {
	type Target = SelectMenu;

	fn build(self) -> Result<Self::Target, BuildError> {
		let custom_id = self.custom_id.ok_or(BuildError)?;
		let disabled = self.disabled.unwrap_or(false);
		let max_values = self.max_values;
		let min_values = self.min_values;
		let options: Result<Vec<_>, _> = self
			.options
			.into_iter()
			.map(ComponentBuilder::build)
			.collect();
		let placeholder = self.placeholder;

		Ok(SelectMenu {
			custom_id,
			disabled,
			max_values,
			min_values,
			options: options?,
			placeholder,
		})
	}

	fn build_component(self) -> Result<Component, BuildError> {
		Ok(Component::SelectMenu(self.build()?))
	}
}

#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct SelectMenuOptionBuilder {
	pub default: Option<bool>,
	pub description: Option<String>,
	pub emoji: Option<ReactionType>,
	pub label: Option<String>,
	pub value: Option<String>,
}

impl SelectMenuOptionBuilder {
	pub fn set_default(&mut self, value: bool) -> &mut Self {
		self.default = Some(value);

		self
	}

	pub fn description(&mut self, value: impl Into<String>) -> &mut Self {
		self.description = Some(value.into());

		self
	}

	pub fn emoji(&mut self, emoji: ReactionType) -> &mut Self {
		self.emoji = Some(emoji);

		self
	}

	pub fn label(&mut self, value: impl Into<String>) -> &mut Self {
		self.label = Some(value.into());

		self
	}

	pub fn value(&mut self, value: impl Into<String>) -> &mut Self {
		self.value = Some(value.into());

		self
	}
}

impl ComponentBuilder for SelectMenuOptionBuilder {
	type Target = SelectMenuOption;

	fn build(self) -> Result<Self::Target, BuildError> {
		let default = self.default.unwrap_or(false);
		let description = self.description;
		let emoji = self.emoji;
		let label = self.label.ok_or(BuildError)?;
		let value = self.value.ok_or(BuildError)?;

		Ok(SelectMenuOption {
			default,
			description,
			emoji,
			label,
			value,
		})
	}

	fn build_component(self) -> Result<Component, BuildError> {
		Err(BuildError)
	}
}

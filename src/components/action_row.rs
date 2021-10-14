#![allow(dead_code)]

use super::{BuildError, ButtonBuilder, ComponentBuilder, SelectMenuBuilder};
use std::iter::FromIterator;
use twilight_model::application::component::{ActionRow, Button, Component, ComponentType};

#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct ActionRowBuilder {
	components: Vec<Component>,
}

impl ActionRowBuilder {
	#[must_use]
	pub const fn new() -> Self {
		Self { components: vec![] }
	}

	#[must_use]
	pub fn len(&self) -> usize {
		self.components.len()
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.components.is_empty()
	}

	pub fn add_menu(&mut self, builder: SelectMenuBuilder) -> Self {
		let menu = builder.build_component().unwrap();

		self.components.push(menu);

		self.clone()
	}

	pub fn add_button(&mut self, builder: ButtonBuilder) -> Self {
		let button = builder.build_component().unwrap();

		self.components.push(button);

		self.clone()
	}

	pub fn create_menu<F: FnOnce(SelectMenuBuilder) -> SelectMenuBuilder>(
		&mut self,
		menu_fn: F,
	) -> Self {
		self.add_menu(menu_fn(SelectMenuBuilder::new()))
	}

	pub fn create_button<F: FnOnce(ButtonBuilder) -> ButtonBuilder>(
		&mut self,
		button_fn: F,
	) -> Self {
		self.add_button(button_fn(ButtonBuilder::new()))
	}

	pub fn push_component(&mut self, button: Component) -> Self {
		self.components.push(button);

		self.clone()
	}

	pub fn push_button(&mut self, button: Button) -> Self {
		self.components.push(Component::Button(button));

		self.clone()
	}
}

impl ComponentBuilder for ActionRowBuilder {
	type Target = ActionRow;

	fn build(self) -> Result<Self::Target, BuildError> {
		if self
			.components
			.iter()
			.any(|component| component.kind() == ComponentType::ActionRow)
		{
			return Err(BuildError::InvalidComponentType);
		}

		Ok(ActionRow {
			components: self.components,
		})
	}

	fn build_component(self) -> Result<Component, BuildError> {
		Ok(Component::ActionRow(self.build()?))
	}
}

impl ComponentBuilder for Vec<Button> {
	type Target = ActionRow;

	fn build(self) -> Result<Self::Target, BuildError> {
		ActionRowBuilder::from(self).build()
	}

	fn build_component(self) -> Result<Component, BuildError> {
		ActionRowBuilder::from(self).build_component()
	}
}

impl FromIterator<Component> for ActionRowBuilder {
	fn from_iter<T: IntoIterator<Item = Component>>(iter: T) -> Self {
		let mut row = Self::new();

		for component in iter {
			row.push_component(component);
		}

		row
	}
}

impl FromIterator<Button> for ActionRowBuilder {
	fn from_iter<T: IntoIterator<Item = Button>>(iter: T) -> Self {
		let mut row = Self::new();

		for component in iter {
			row.push_button(component);
		}

		row
	}
}

impl From<Vec<Button>> for ActionRowBuilder {
	fn from(buttons: Vec<Button>) -> Self {
		Self {
			components: buttons.into_iter().map(Component::Button).collect(),
		}
	}
}

impl<const N: usize> ComponentBuilder for [Button; N] {
	type Target = ActionRow;

	fn build(self) -> Result<Self::Target, BuildError> {
		ActionRowBuilder::from(self).build()
	}

	fn build_component(self) -> Result<Component, BuildError> {
		ActionRowBuilder::from(self).build_component()
	}
}

impl<const N: usize> From<[Button; N]> for ActionRowBuilder {
	fn from(buttons: [Button; N]) -> Self {
		Self {
			components: buttons.iter().cloned().map(Component::Button).collect(),
		}
	}
}

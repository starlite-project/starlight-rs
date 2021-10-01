#![allow(dead_code)]

use super::{BuildError, ComponentBuilder};
use twilight_model::{
	application::component::{button::ButtonStyle, Button, Component},
	channel::ReactionType,
};

#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct ButtonBuilder {
	custom_id: Option<String>,
	disabled: Option<bool>,
	emoji: Option<ReactionType>,
	label: Option<String>,
	style: Option<ButtonStyle>,
	url: Option<String>,
}

impl ButtonBuilder {
	#[must_use]
	pub const fn new() -> Self {
		Self {
			custom_id: None,
			disabled: None,
			emoji: None,
			label: None,
			style: None,
			url: None,
		}
	}

	pub fn custom_id(&mut self, value: impl Into<String>) -> Self {
		self.custom_id = Some(value.into());

		self.clone()
	}

	pub fn set_disabled(&mut self, value: bool) -> Self {
		self.disabled = Some(value);

		self.clone()
	}

	pub fn label(&mut self, label: impl Into<String>) -> Self {
		self.label = Some(label.into());

		self.clone()
	}

	pub fn disable(&mut self) -> Self {
		self.set_disabled(true)
	}

	pub fn enable(&mut self) -> Self {
		self.set_disabled(false)
	}

	pub fn style(&mut self, style: ButtonStyle) -> Self {
		self.style = Some(style);

		self.clone()
	}

	pub fn url(&mut self, link: impl Into<String>) -> Self {
		self.url = Some(link.into());

		self.style(ButtonStyle::Link)
	}
}

impl ComponentBuilder for ButtonBuilder {
	type Target = Button;

	fn build(self) -> Result<Self::Target, BuildError> {
		let custom_id = self.custom_id;
		let disabled = self.disabled.unwrap_or(false);
		let emoji = self.emoji;
		let label = self.label;
		let style = self.style.ok_or(BuildError::ValueNotSet("style"))?;
		let url = self.url;

		Ok(Button {
			custom_id,
			disabled,
			emoji,
			label,
			style,
			url,
		})
	}

	fn build_component(self) -> Result<Component, BuildError> {
		Ok(Component::Button(self.build()?))
	}
}

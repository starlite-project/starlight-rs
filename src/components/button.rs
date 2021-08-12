#![allow(dead_code)]

use super::{ComponentBuilder, builder::BuildError};
use twilight_model::{application::component::{Button, Component, button::ButtonStyle}, channel::ReactionType};

#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct ButtonBuilder {
    custom_id: Option<String>,
    disabled: Option<bool>,
    emoji: Option<ReactionType>,
    label: Option<String>,
    style: Option<ButtonStyle>,
    url: Option<String>
}

impl ButtonBuilder {
    pub const fn new() -> Self {
        Self {
            custom_id: None,
            disabled: None,
            emoji: None,
            label: None,
            style: None,
            url: None
        }
    }

    pub fn custom_id(&mut self, value: impl Into<String>) -> &mut Self {
        self.custom_id = Some(value.into());

        self
    }

    pub fn set_disabled(&mut self, value: bool) -> &mut Self {
        self.disabled = Some(value);

        self
    }

    pub fn label(&mut self, label: impl Into<String>) -> &mut Self {
        self.label = Some(label.into());

        self
    }

    pub fn disable(&mut self) -> &mut Self {
        self.set_disabled(true)
    }

    pub fn enable(&mut self) -> &mut Self {
        self.set_disabled(false)
    }

    pub fn style(&mut self, style: ButtonStyle) -> &mut Self {
        self.style = Some(style);

        self
    }

    pub fn url(&mut self, link: impl Into<String>) -> &mut Self {
        self.url = Some(link.into());

        self.style(ButtonStyle::Link)
    }
}

impl ComponentBuilder for ButtonBuilder {
    type Target = Button;

    fn build(self) -> Result<Self::Target, BuildError> {
        let custom_id = self.custom_id;
        let disabled = self.disabled.ok_or(BuildError)?;
        let emoji = self.emoji;
        let label = self.label;
        let style = self.style.ok_or(BuildError)?;
        let url = self.url;

        Ok(Button {
            custom_id,
            disabled,
            emoji,
            label,
            style,
            url
        })
    }

    fn build_component(self) -> Result<Component, BuildError> {
        Ok(Component::Button(self.build()?))
    }
}
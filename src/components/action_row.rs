#![allow(dead_code)]

use super::{BuildError, ButtonBuilder, ComponentBuilder};
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

    pub fn add_button(&mut self, builder: ButtonBuilder) -> Self {
        let button = builder.build_component().unwrap();

        self.components.push(button);

        self.clone()
    }

    pub fn create_button<F: FnOnce(ButtonBuilder) -> ButtonBuilder>(
        &mut self,
        button_fn: F,
    ) -> Self {
        self.add_button(button_fn(ButtonBuilder::new()))
    }

    pub fn push_button(&mut self, button: Component) -> Self {
        self.components.push(button);

        self.clone()
    }
}

impl ComponentBuilder for ActionRowBuilder {
    type Target = ActionRow;

    fn build(self) -> Result<Self::Target, BuildError> {
        if !self
            .components
            .iter()
            .all(|component| component.kind() == ComponentType::Button)
        {
            return Err(BuildError);
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

impl From<Vec<Button>> for ActionRowBuilder {
    fn from(buttons: Vec<Button>) -> Self {
        Self {
            components: buttons.into_iter().map(Component::Button).collect(),
        }
    }
}

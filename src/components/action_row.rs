#![allow(dead_code)]

use super::{BuildError, ButtonBuilder, ComponentBuilder};
use twilight_model::application::component::{ActionRow, Component, ComponentType};

#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct ActionRowBuilder {
    components: Vec<Component>,
}

impl ActionRowBuilder {
    pub const fn new() -> Self {
        Self { components: vec![] }
    }

    pub fn add_button(&mut self, builder: ButtonBuilder) -> Self {
        let button = builder.build_component().unwrap();

        self.components.push(button);

        self.clone()
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

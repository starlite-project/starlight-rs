use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::application::component::Component;

pub trait ComponentBuilder {
    type Target;

    fn build(self) -> Result<Self::Target, BuildError>;

    fn build_component(self) -> Result<Component, BuildError>;
}

#[derive(Debug)]
pub struct BuildError;

impl Display for BuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("an error occured while building the component")
    }
}

impl Error for BuildError {}
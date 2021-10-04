use crate::slashies::interaction::Interaction;
use super::ClickCommand;
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy)]
#[error("an error occurred parsing the data")]
pub struct ParseError;

pub trait ParseCommand<const N: usize>: ClickCommand<N> {
    type Output;

    fn parse(interaction: Interaction, input: &str) -> Result<Self::Output, ParseError>;
}
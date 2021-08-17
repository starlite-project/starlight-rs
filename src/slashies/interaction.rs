use twilight_http::{
    request::application::{InteractionError, UpdateOriginalResponse},
    Error,
};
use twilight_model::application::{callback::InteractionResponse, interaction::ApplicationCommand};

use crate::state::State;

use super::Response;

#[derive(Debug, Clone, Copy)]
pub struct Interaction<'a> {
    pub state: State,
    // Use a reference bc Interaction doesn't need to own it
    pub command: &'a ApplicationCommand,
}

impl<'a> Interaction<'a> {
    pub async fn ack(&self) -> Result<(), Error> {
        self.state
            .http
            .interaction_callback(self.command.id, &self.command.token, &Response::ack())
            .exec()
            .await?;
        Ok(())
    }

    pub async fn response<T: Send + Sync + Into<InteractionResponse>>(
        &self,
        response: T,
    ) -> Result<(), Error> {
        self.state
            .http
            .interaction_callback(self.command.id, &self.command.token, &response.into())
            .exec()
            .await?;
        Ok(())
    }

    pub fn update(&self) -> Result<UpdateOriginalResponse<'_>, InteractionError> {
        self.state
            .http
            .update_interaction_original(&self.command.token)
    }
}

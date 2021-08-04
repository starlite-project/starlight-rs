use twilight_http::Error;
use twilight_model::application::{callback::InteractionResponse, interaction::ApplicationCommand};

use crate::state::State;

use super::Response;

pub struct Interaction {
    pub state: State,
    pub command: ApplicationCommand,
}

impl Interaction {
    pub async fn ack(&self) -> Result<(), Error> {
        self.state
            .http
            .interaction_callback(self.command.id, &self.command.token, &Response::ack())
            .exec()
            .await?;
        Ok(())
    }

    pub async fn response(&self, response: &InteractionResponse) -> Result<(), Error> {
        self.state
            .http
            .interaction_callback(self.command.id, &self.command.token, response)
            .exec()
            .await?;
        Ok(())
    }
}

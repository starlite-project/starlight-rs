use super::Response;
use crate::{persistence::Database, state::State};
use miette::{IntoDiagnostic, Result as MietteResult};
use twilight_http::{request::application::interaction::UpdateOriginalResponse, Error};
use twilight_model::{
	application::{callback::InteractionResponse, interaction::ApplicationCommand},
	channel::Message,
};

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

	pub async fn get_message(&self) -> MietteResult<Message> {
		let get_original_response = self
			.state
			.http
			.get_interaction_original(&self.command.token)
			.into_diagnostic()?;

		Ok(supernova::model!(@diagnostic get_original_response))
	}

	pub fn update(&self) -> MietteResult<UpdateOriginalResponse<'_>> {
		self.state
			.http
			.update_interaction_original(&self.command.token)
			.into_diagnostic()
	}

	#[must_use]
	pub fn database(&'a self) -> &'a Database {
		&self.state.database
	}
}

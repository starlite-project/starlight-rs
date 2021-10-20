use super::Response;
use crate::state::State;
use miette::{IntoDiagnostic, Result as MietteResult};
use serde_json::to_string;
use supernova::model;
use twilight_http::Error;
use twilight_model::{
	application::{
		callback::{CallbackData, InteractionResponse},
		interaction::ApplicationCommand,
	},
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

		model!(get_original_response).await.into_diagnostic()
	}

	pub async fn update<T: Send + Sync + Into<CallbackData>>(
		&self,
		response: T,
	) -> MietteResult<()> {
		let callback_data: CallbackData = response.into();
		let update_interaction = self
			.state
			.http
			.update_interaction_original(&self.command.token)
			.into_diagnostic()?;

		let bytes = to_string(&callback_data).into_diagnostic()?;

		update_interaction
			.payload_json(bytes.as_bytes())
			.exec()
			.await
			.into_diagnostic()?;

		Ok(())
	}
}

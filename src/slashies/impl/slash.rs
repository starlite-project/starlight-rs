use crate::state::State;
use async_trait::async_trait;
use miette::Result;
use twilight_model::application::command::Command;

#[async_trait]
pub trait SlashCommand {
	const NAME: &'static str;

	fn define() -> Command;

	async fn run(&self, state: State) -> Result<()>;
}

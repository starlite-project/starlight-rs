use crate::slashies::interaction::Interaction;
use async_trait::async_trait;
use miette::Result;
use twilight_util::builder::command::CommandBuilder;

#[async_trait]
pub trait SlashCommand {
	const NAME: &'static str;

	fn define() -> CommandBuilder;

	async fn run(&self, interaction: Interaction<'_>) -> Result<()>;
}

use super::{interaction::Interaction, SlashCommand};
use crate::utils::CacheReliant;
use click::Click;
use info::Info;
use miette::Result;
use ping::Ping;
use stats::Stats;
use twilight_cache_inmemory::ResourceType;
use twilight_model::application::{command::Command, interaction::ApplicationCommand};

mod click;
mod info;
mod ping;
mod stats;

#[must_use]
pub fn get_slashies() -> [Command; 4] {
	[
		Ping::define(),
		Click::define(),
		Info::define(),
		Stats::define(),
	]
}

#[derive(Debug, Clone)]
pub enum Commands {
	Ping(Ping),
	Click(Click),
	Info(Info),
	Stats(Stats),
}

impl Commands {
	#[must_use]
	pub fn r#match(command: &ApplicationCommand) -> Option<Self> {
		match command.data.name.as_str() {
			Ping::NAME => Some(Self::Ping(Ping(command.clone()))),
			Click::NAME => Some(Self::Click(Click(command.clone()))),
			Info::NAME => Some(Self::Info(Info(command.clone()))),
			Stats::NAME => Some(Self::Stats(Stats(command.clone()))),
			_ => None,
		}
	}

	pub async fn run(&self, interaction: Interaction<'_>) -> Result<()> {
		match self {
			Self::Ping(c) => c.run(interaction).await,
			Self::Click(c) => c.run(interaction).await,
			Self::Info(c) => c.run(interaction).await,
			Self::Stats(c) => c.run(interaction).await,
		}
	}
}

impl CacheReliant for Commands {
	fn needs() -> ResourceType {
		Info::needs() | Stats::needs()
	}
}

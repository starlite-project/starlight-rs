use super::SlashCommand;
use crate::{state::State, utils::CacheReliant};
use click::Click;
use info::Info;
use miette::Result;
use ping::Ping;
use settings::Settings;
use stats::Stats;
use twilight_cache_inmemory::ResourceType;
use twilight_model::application::{command::Command, interaction::ApplicationCommand};

mod click;
mod info;
mod ping;
mod settings;
mod stats;

#[must_use]
pub fn get_slashies() -> [Command; 5] {
	[
		Ping::define(),
		Click::define(),
		Info::define(),
		Stats::define(),
		Settings::define(),
	]
}

#[derive(Debug, Clone)]
pub enum Commands {
	Ping(Ping),
	Click(Click),
	Info(Info),
	Stats(Stats),
	Settings(Settings),
}

impl Commands {
	#[must_use]
	pub fn r#match(command: ApplicationCommand) -> Option<Self> {
		match command.data.name.as_str() {
			Ping::NAME => Some(Self::Ping(Ping(command))),
			Click::NAME => Some(Self::Click(Click(command))),
			Info::NAME => Some(Self::Info(Info(command))),
			Stats::NAME => Some(Self::Stats(Stats(command))),
			Settings::NAME => Some(Self::Settings(Settings(command))),
			_ => None,
		}
	}

	pub async fn run(&self, state: State) -> Result<()> {
		match self {
			Self::Ping(c) => c.run(state).await,
			Self::Click(c) => c.run(state).await,
			Self::Info(c) => c.run(state).await,
			Self::Stats(c) => c.run(state).await,
			Self::Settings(c) => c.run(state).await,
		}
	}
}

impl CacheReliant for Commands {
	fn needs() -> ResourceType {
		Info::needs() | Settings::needs() | Stats::needs()
	}
}

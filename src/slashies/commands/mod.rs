use super::interaction::Interaction;
use crate::{
	components::{BuildError, ComponentBuilder},
	state::State,
	utils::CacheReliant,
};
use async_trait::async_trait;
use base64::encode;
use click::Click;
use info::Info;
use miette::{IntoDiagnostic, Result};
use ping::Ping;
use settings::Settings;
use stats::Stats;
use std::{
	any::type_name,
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
	lazy::Lazy,
};
use twilight_cache_inmemory::ResourceType;
use twilight_model::{
	application::{
		command::Command,
		component::{Button, Component},
		interaction::{
			ApplicationCommand, Interaction as DiscordInteraction, MessageComponentInteraction,
		},
	},
	gateway::event::Event,
	id::UserId,
};

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

#[async_trait]
pub trait SlashCommand<const N: usize> {
	const NAME: &'static str;

	fn define() -> Command;

	async fn run(&self, state: State) -> Result<()>;
}

#[async_trait]
pub trait ClickCommand<const N: usize>: SlashCommand<N> {
	const COMPONENT_IDS: Lazy<[&'static str; N]> = Lazy::new(|| {
		let mut array = [""; N];

		let name = type_name::<Self>()
			.split("::")
			.last()
			.unwrap_or_else(|| supernova::debug_unreachable!());

		let encoded = encode(name);

		for (i, val) in array.iter_mut().enumerate() {
			*val = Box::leak(Box::new(format!("{}_{}", encoded, i)));
		}

		array
	});

	type Output;

	const EMPTY_COMPONENTS: Option<&'static [Component]> = Some(&[]);

	fn define_buttons() -> Result<[Button; N], BuildError>;

	fn parse(interaction: Interaction, input: &str) -> Self::Output;

	fn components() -> Result<Vec<Component>, BuildError> {
		Ok(vec![Self::define_buttons()?.build_component()?])
	}

	async fn wait_for_click<'a>(
		state: State,
		interaction: Interaction<'a>,
		user_id: UserId,
	) -> Result<MessageComponentInteraction> {
		let waiter = move |event: &Event| {
			if let Event::InteractionCreate(interaction_create) = event {
				if let DiscordInteraction::MessageComponent(ref button) = interaction_create.0 {
					if Self::COMPONENT_IDS.contains(&button.data.custom_id.as_str())
						&& button.author_id().unwrap_or_default() == user_id
					{
						return true;
					}
				}
			}

			false
		};

		let event = if let Some(guild_id) = interaction.command.guild_id {
			state
				.standby
				.wait_for(guild_id, waiter)
				.await
				.into_diagnostic()?
		} else {
			state
				.standby
				.wait_for_event(waiter)
				.await
				.into_diagnostic()?
		};

		if let Event::InteractionCreate(interaction_create) = event {
			if let DiscordInteraction::MessageComponent(comp) = interaction_create.0 {
				Ok(*comp)
			} else {
				Err(miette::miette!(ClickError))
			}
		} else {
			Err(miette::miette!(ClickError))
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct SlashError;

impl Display for SlashError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("an error occurred during the slash command's execution")
	}
}

impl Error for SlashError {}

#[derive(Debug, Clone, Copy)]
pub struct ClickError;

impl Display for ClickError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("an error occurred getting data from the interaction")
	}
}

impl Error for ClickError {}

impl<T: SlashCommand<0>> !ClickCommand<0> for T {}

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

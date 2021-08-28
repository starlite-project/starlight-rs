use super::SlashCommand;
use crate::{
	slashies::{interaction::Interaction, Response},
	state::State,
};
use anyhow::Result;
use async_trait::async_trait;
use std::{
	cmp::min,
	convert::TryFrom,
	error::Error,
	fmt::{Display, Error as FmtError, Formatter, Result as FmtResult},
	fs::metadata,
};
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};
use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder};
use twilight_model::application::{command::Command, interaction::ApplicationCommand};

#[derive(Debug)]
struct ConvertError(u64);

impl Display for ConvertError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("cannot convert ")?;
		Display::fmt(&self.0, f)?;
		f.write_str(" to f64")
	}
}

impl Error for ConvertError {}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Bytes(f64);

impl Bytes {
	const UNITS: [&'static str; 9] = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
}

impl Display for Bytes {
	#[allow(
		clippy::cast_sign_loss,
		clippy::cast_possible_truncation,
		clippy::cast_possible_wrap
	)]
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let negative = if self.0.is_sign_positive() { "" } else { "-" };
		let num = self.0.abs();
		if num < 1.0 {
			Display::fmt(&negative, f)?;
			Display::fmt(&num, f)?;
			return f.write_str(" B");
		}
		let delimiter = 1000_f64;
		let exponent = min(
			num.log(delimiter).floor() as i32,
			(Self::UNITS.len() - 1) as i32,
		);
		let pretty_bytes = format!("{:.2}", num / delimiter.powi(exponent))
			.parse::<f64>()
			.map_err(|_| FmtError)?
			* 1.0;
		let unit = Self::UNITS[exponent as usize];
		Display::fmt(&negative, f)?;
		Display::fmt(&pretty_bytes, f)?;
		f.write_str(" ")?;
		Display::fmt(&unit, f)
	}
}

impl TryFrom<u64> for Bytes {
	type Error = ConvertError;

	#[allow(
		clippy::cast_sign_loss,
		clippy::cast_possible_truncation,
		clippy::cast_precision_loss
	)]
	fn try_from(value: u64) -> Result<Self, Self::Error> {
		let result = value as f64;
		if result as u64 != value {
			return Err(ConvertError(value));
		}
		Ok(Self(result))
	}
}

#[derive(Debug, Clone)]
pub struct Stats(pub(super) ApplicationCommand);

impl Stats {
	fn statistics(&self, interaction: Interaction) -> String {
		let cache_stats = interaction.state.cache.stats();

		let channels_size = {
			let current_guild_count = interaction
				.state
				.cache
				.guild_channels(interaction.command.guild_id.unwrap_or_default())
				.unwrap_or_default()
				.len();

			cache_stats.groups() + cache_stats.private_channels() + current_guild_count
		};

		let guilds = cache_stats.guilds();
		let users = cache_stats.users();

		let rustc_version = {
			let mut version = crate::build_info::RUSTC_VERSION.to_string();

			let range = version.find('(').unwrap_or(version.len());

			version.replace_range(range.., "");

			version
		};

		format!("**• Users:** {users}\n**• Servers:** {guilds}\n**• Channels:** {channels}\n**• Starlight version:** {crate_version}\n**• Rust version:** {rust_version}", users = users, guilds = guilds, channels = channels_size, crate_version = crate::build_info::PKG_VERSION, rust_version = rustc_version)
	}
}

#[async_trait]
impl SlashCommand<0> for Stats {
	const NAME: &'static str = "stats";

	fn define() -> Command {
		Command {
			application_id: None,
			guild_id: None,
			name: String::from(Self::NAME),
			default_permission: None,
			description: String::from("Get the stats for the bot"),
			id: None,
			options: vec![],
		}
	}

	async fn run(&self, state: State) -> Result<()> {
		let interaction = state.interaction(&self.0);

		let system = System::new_all();
		let current_process = system
			.process(get_current_pid().expect("failed to get pid"))
			.unwrap_or_else(|| crate::debug_unreachable!());

		let binary_path = current_process.exe();

		let binary_size = Bytes::try_from(metadata(binary_path)?.len())?;

		let runtime = state.runtime.elapsed();
		let now = chrono::Utc::now();

		dbg!(runtime - now);
		dbg!(runtime);

		let embed = EmbedBuilder::new()
			.color(crate::helpers::STARLIGHT_PRIMARY_COLOR.to_decimal())
			.field(EmbedFieldBuilder::new("Statistics", self.statistics(interaction)))
			.field(EmbedFieldBuilder::new("Uptime", String::from("todo")))
			.field(EmbedFieldBuilder::new("Server Usage", String::from("todo")));

		interaction.response(Response::from(embed.build()?)).await?;

		Ok(())
	}
}

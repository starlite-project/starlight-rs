use super::SlashCommand;
use crate::{
	slashies::{interaction::Interaction, Response},
	state::State,
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use std::{
	cmp::min,
	convert::{TryFrom, TryInto},
	error::Error,
	fmt::{Display, Error as FmtError, Formatter, Result as FmtResult},
	fs::metadata,
	lazy::Lazy,
	time::Duration as StdDuration,
};
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};
use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder};
use twilight_model::application::{
	command::{Command, CommandType},
	interaction::ApplicationCommand,
};

const DOT: &str = "\u{2022}";

#[cfg(debug_assertions)]
const BUILD_TYPE: &str = "Debug";

#[cfg(not(debug_assertions))]
const BUILD_TYPE: &str = "Release";

#[allow(clippy::declare_interior_mutable_const)]
const BUILD_SIZE: Lazy<Bytes> = Lazy::new(|| {
	let system = System::new_all();

	let process = system
		.process(get_current_pid().expect("failed to get pid"))
		.expect("failed to get process");

	let path = process.exe();

	Bytes::try_from(metadata(path).expect("failed to get metadata").len())
		.expect("failed to get byte size")
});

#[derive(Debug)]
struct OutOfRangeError;

impl Display for OutOfRangeError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("source duration value is out of range for the target type")
	}
}

impl Error for OutOfRangeError {}

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
#[repr(transparent)]
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
		let delimiter = 1024_f64;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Uptime(Duration);

impl Display for Uptime {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		if self.0.num_weeks() > 0 {
			Display::fmt(&self.0.num_weeks(), f)?;
			f.write_str(" weeks ")?;
			Display::fmt(&(self.0.num_days() - (self.0.num_weeks() * 7)), f)?;
			f.write_str(" days")
		} else if self.0.num_days() > 0 && self.0.num_days() <= 7 {
			Display::fmt(&self.0.num_days(), f)?;
			f.write_str(" days ")?;
			Display::fmt(&(self.0.num_hours() - (self.0.num_days() * 24)), f)?;
			f.write_str(" hours")
		} else if self.0.num_hours() > 0 && self.0.num_hours() <= 24 {
			Display::fmt(&self.0.num_hours(), f)?;
			f.write_str(" hours ")?;
			Display::fmt(&(self.0.num_minutes() - (self.0.num_hours() * 60)), f)?;
			f.write_str(" minutes")
		} else {
			Display::fmt(&self.0.num_minutes(), f)?;
			f.write_str(" minutes ")?;
			Display::fmt(&(self.0.num_seconds() - (self.0.num_minutes() * 60)), f)?;
			f.write_str(" seconds")
		}
	}
}

impl From<Duration> for Uptime {
	fn from(duration: Duration) -> Self {
		Self(duration)
	}
}

impl TryFrom<StdDuration> for Uptime {
	type Error = OutOfRangeError;

	fn try_from(value: StdDuration) -> Result<Self, Self::Error> {
		match Duration::from_std(value) {
			Ok(duration) => Ok(Self(duration)),
			Err(_) => Err(OutOfRangeError),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Stats(pub(super) ApplicationCommand);

impl Stats {
	fn statistics(interaction: Interaction) -> String {
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

			let range = version.find('(').unwrap_or_else(|| version.len())..;

			version.replace_range(range, "");

			version
		};

		format!("**{dot} Users:** {users}\n**{dot} Servers:** {guilds}\n**{dot} Channels:** {channels}\n**{dot} Rust:** {rust_version}\n**{dot} Build type:** {build_type}", users = users, guilds = guilds, channels = channels_size, rust_version = rustc_version, dot = DOT, build_type = BUILD_TYPE)
	}

	fn uptime(interaction: Interaction) -> Result<String> {
		let system = System::new();
		let bot_uptime: Uptime = interaction.state.runtime.elapsed().try_into()?;

		let host_uptime: Uptime = StdDuration::from_secs(system.uptime()).try_into()?;

		Ok(format!(
			"**{dot} Host:** {host_uptime}\n**{dot} Client:** {bot_uptime}",
			host_uptime = host_uptime,
			bot_uptime = bot_uptime,
			dot = DOT
		))
	}

	#[allow(clippy::cast_precision_loss, clippy::borrow_interior_mutable_const)]
	async fn server_usage() -> Result<String> {
		let cpu_count = num_cpus::get_physical() as f64;
		let process = crate::utils::get_current_process()?;

		process.cpu_usage();
		tokio::time::sleep(StdDuration::from_millis(200)).await;
		let cpu_usage = (f64::from(process.cpu_usage()) / cpu_count) * 100.0;

		let memory_usage = Bytes::try_from(star_utils::memory()?)?;

		if cfg!(windows) {
			Ok(format!("**{dot} CPU Usage:** {cpu_usage:.2}%\n**{dot} Memory usage:** {memory_usage}\n**{dot} Binary size:** {binary_size}", dot = DOT, cpu_usage = cpu_usage, memory_usage = memory_usage, binary_size = *BUILD_SIZE))
		} else {
			Ok(format!("**{dot} CPU Usage:** {cpu_usage:.2}%\n**{dot} Memory Allocated:** {memory_usage}\n**{dot} Binary size:** {binary_size}", dot = DOT, cpu_usage = cpu_usage, memory_usage = memory_usage, binary_size = *BUILD_SIZE))
		}
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
			kind: CommandType::ChatInput,
		}
	}

	async fn run(&self, state: State) -> Result<()> {
		let interaction = state.interaction(&self.0);

		let embed = EmbedBuilder::new()
			.color(crate::helpers::STARLIGHT_PRIMARY_COLOR.to_decimal())
			.field(EmbedFieldBuilder::new(
				"Statistics",
				Self::statistics(interaction),
			))
			.field(EmbedFieldBuilder::new("Uptime", Self::uptime(interaction)?))
			.field(EmbedFieldBuilder::new(
				"Server Usage",
				Self::server_usage().await?,
			));

		interaction.response(Response::from(embed.build()?)).await?;

		Ok(())
	}
}

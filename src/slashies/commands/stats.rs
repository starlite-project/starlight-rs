use super::SlashCommand;
use crate::{slashies::Response, state::State};
use anyhow::Result;
use async_trait::async_trait;
use std::{
	cmp::min,
	convert::TryFrom,
	error::Error,
	fmt::{Display, Error as FmtError, Formatter, Result as FmtResult},
};
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
			(num.ln() / delimiter.ln()).floor() as i32,
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

		let binary_size = Bytes::try_from(crate::get_binary_metadata()?.len())?;

		dbg!(binary_size);

		let pid = std::process::id();

		interaction
			.response(Response::from(format!(
				"Binary size: {}\nProcess ID: {}",
				binary_size, pid
			)))
			.await?;

		Ok(())
	}
}

use super::SlashCommand;
use crate::state::State;
use anyhow::Result;
use async_trait::async_trait;
use std::fmt::{Display, Formatter, Result as FmtResult};
use twilight_model::application::{command::Command, interaction::ApplicationCommand};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum BinarySize {
	Bytes(u64),
	Kilo(u64),
	Mega(u64),
	Giga(u64),
}

impl Display for BinarySize {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match *self {
			Self::Bytes(bytes) => {
				bytes.fmt(f)?;
				f.write_str(" bytes")
			}
			Self::Kilo(bytes) => {
				bytes.fmt(f)?;
				f.write_str(" KB")
			}
			Self::Mega(bytes) => {
				bytes.fmt(f)?;
				f.write_str(" MB")
			}
			Self::Giga(bytes) => {
				bytes.fmt(f)?;
				f.write_str(" GB")
			}
		}
	}
}

impl From<u64> for BinarySize {
	fn from(bytes: u64) -> Self {
		if (bytes / 1024) == 0 {
			Self::Bytes(bytes)
		} else if (bytes / 1024 / 1024) == 0 {
			Self::Kilo(bytes / 1024)
		} else if (bytes / 1024 / 1024 / 1024) == 0 {
			Self::Mega(bytes / 1024 / 1024)
		} else {
			Self::Giga(bytes / 1024 / 1024 / 1024)
		}
	}
}

impl Into<u64> for BinarySize {
	fn into(self) -> u64 {
		match self {
			Self::Bytes(bytes) => bytes,
			Self::Kilo(bytes) => bytes * 1024,
			Self::Mega(bytes) => bytes * 1024 * 1024,
			Self::Giga(bytes) => bytes * 1024 * 1024 * 1024,
		}
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

		let binary_size: BinarySize = crate::get_binary_metadata()?.len().into();

		let pid = std::process::id();

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::BinarySize;
	use static_assertions::assert_impl_all;
	use std::{
		fmt::{Debug, Display},
		hash::Hash,
	};

	assert_impl_all!(
		BinarySize: Clone,
		Copy,
		Debug,
		Display,
		Eq,
		Hash,
		Ord,
		PartialEq,
		PartialOrd
	);

	const BYTES: u64 = 512;
	const KILO: u64 = 2048;
	const MEGA: u64 = 3_145_728;
	const GIGA: u64 = 10_737_418_240;

	#[test]
	fn from_u64() {
		assert_eq!(BinarySize::from(BYTES), BinarySize::Bytes(512));
		assert_eq!(BinarySize::from(KILO), BinarySize::Kilo(2));
		assert_eq!(BinarySize::from(MEGA), BinarySize::Mega(3));
		assert_eq!(BinarySize::from(GIGA), BinarySize::Giga(10));
	}

	#[test]
	fn into_u64() {
		let bytes: u64 = BinarySize::Bytes(BYTES).into();
		let kilo: u64 = BinarySize::Kilo(2).into();
		let mega: u64 = BinarySize::Mega(3).into();
		let giga: u64 = BinarySize::Giga(10).into();

		assert_eq!(bytes, BYTES);
		assert_eq!(kilo, KILO);
		assert_eq!(mega, MEGA);
		assert_eq!(giga, GIGA);
	}
}

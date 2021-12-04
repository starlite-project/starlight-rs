use std::pin::Pin;

use futures_util::Future;
use reqwest::{header, Client};
use serde::Deserialize;
use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::datetime::Timestamp;

use crate::{
	helpers::{InteractionsHelper, STARLIGHT_COLORS},
	prelude::*,
	slashies::{SlashCommand, SlashData},
};

const USER_AGENT: &str = "pyrotechniac/starlight";

#[derive(Debug)]
enum CrateResult {
	Found(CrateInfo),
	NotFound(String),
}

impl From<CrateInfo> for CrateResult {
	fn from(info: CrateInfo) -> Self {
		Self::Found(info)
	}
}

impl From<String> for CrateResult {
	fn from(not_found: String) -> Self {
		Self::NotFound(not_found)
	}
}

#[derive(Debug, Deserialize)]
struct CrateList {
	crates: Vec<CrateInfo>,
}

#[derive(Debug, Deserialize)]
struct CrateInfo {
	id: String,
	name: String,
	newest_version: String,
	updated_at: String,
	downloads: u64,
	description: Option<String>,
	documentation: Option<String>,
	exact_match: bool,
}

#[derive(Debug, Clone, CreateCommand, CommandModel)]
#[command(name = "crate", desc = "Lookup crates on crates.io")]
pub struct Crate {
	#[command(desc = "The name of the crate to search for")]
	crate_name: String,
}

impl Crate {
	fn rustc_crate_link(&self) -> Option<&'static str> {
		match self.crate_name.to_ascii_lowercase().as_str() {
			"std" => Some("https://doc.rust-lang.org/stable/std"),
			"core" => Some("https://doc.rust-lang.org/stable/core"),
			"alloc" => Some("https://doc.rust-lang.org/stable/alloc"),
			"proc_macro" => Some("https://doc.rust-lang.org/stable/proc_macro"),
			"beta" => Some("https://doc.rust-lang.org/beta/std"),
			"nightly" => Some("https://doc.rust-lang.org/nightly/std"),
			"rustc" => Some("https://doc.rust-lang.org/nightly/nightly-rustc"),
			"test" => Some("https://doc.rust-lang.org/stable/test"),
			_ => None,
		}
	}

	fn get_documentation(krate: &CrateInfo) -> String {
		krate
			.documentation
			.as_ref()
			.map_or_else(|| format!("https://docs.rs/{}", krate.name), Clone::clone)
	}

	fn format_number(mut n: u64) -> String {
		let mut output = String::new();
		while n >= 1000 {
			output.insert_str(0, &format!(" {:03}", n % 1000));
			n /= 1000;
		}

		output.insert_str(0, &format!("{}", n));
		output
	}

	async fn get_crate(&self) -> MietteResult<CrateResult> {
		event!(Level::INFO, "searching for crate `{}`", &self.crate_name);

		let reqwest_client = Client::builder().build().into_diagnostic()?;

		let crate_list = reqwest_client
			.get("https://crates.io/api/v1/crates")
			.header(header::USER_AGENT, USER_AGENT)
			.query(&[("q", self.crate_name.as_str())])
			.send()
			.await
			.into_diagnostic()?
			.json::<CrateList>()
			.await
			.into_diagnostic()?;

		let krate = crate_list
			.crates
			.into_iter()
			.next()
			.ok_or_else(|| error!("crate `{}` not found", self.crate_name.as_str()))?;

		if krate.exact_match {
			Ok(krate.into())
		} else {
			Ok(format!(
				"Crate `{}` not found. Did you mean `{}`?",
				&self.crate_name.as_str(),
				krate.name
			)
			.into())
		}
	}
}

impl SlashCommand for Crate {
	fn run(
		&self,
		helper: InteractionsHelper,
		mut responder: SlashData,
	) -> Pin<Box<dyn Future<Output = MietteResult<()>> + Send + '_>> {
		Box::pin(async move {
			if let Some(stdlib_url) = self.rustc_crate_link() {
				responder.message(stdlib_url);

				helper.respond(&responder).await.into_diagnostic()?;

				return Ok(());
			}

			let krate = self.get_crate().await?;

			match krate {
				CrateResult::NotFound(msg) => {
					responder.message(msg);
				}
				CrateResult::Found(info) => {
					let embed_builder = EmbedBuilder::new()
						.color(STARLIGHT_COLORS[2].to_decimal())
						.title(self.crate_name.as_str())
						.url(Self::get_documentation(&info))
						.description(
							&info
								.description
								.unwrap_or_else(|| "_<no description available>_".to_owned()),
						)
						.field(EmbedFieldBuilder::new("Version", &info.newest_version).inline())
						.field(EmbedFieldBuilder::new("Downloads", Self::format_number(info.downloads)).inline())
						.timestamp(Timestamp::parse(info.updated_at.as_str()).into_diagnostic()?);

					responder.embed(embed_builder.build().into_diagnostic()?);
				}
			}

			helper.respond(&responder).await.into_diagnostic()?;

			Ok(())
		})
	}
}

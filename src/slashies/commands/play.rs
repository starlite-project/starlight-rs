use std::pin::Pin;

use futures_util::Future;
use twilight_http::request::AttachmentFile;
use twilight_model::application::{
	command::CommandType,
	interaction::application_command::{CommandData, CommandOptionValue},
};
use twilight_util::builder::command::{BooleanBuilder, CommandBuilder, StringBuilder};

use crate::{
	helpers::{
		parsing::CodeBlock,
		playground::{
			get_gist, BuildMode, CrateType, Edition, PlaygroundRequest, PlaygroundResponse,
			ResultHandling, RustChannel,
		},
		InteractionsHelper,
	},
	prelude::*,
	slashies::{DefineCommand, SlashCommand, SlashData},
};

fn single_to_tuple_of_strings<T: Copy + ToString>(value: T) -> (String, String) {
	(value.to_string(), value.to_string())
}

#[derive(Debug, Clone)]
pub struct Play {
	channel: RustChannel,
	mode: BuildMode,
	edition: Edition,
	code: CodeBlock,
	warn: bool,
}

impl Play {
	fn url_from_gist(&self, gist_id: &str) -> String {
		format!(
			"https://play.rust-lang.org/?version={}&mode={}&edition={}&gist={}",
			self.channel, self.mode, self.edition, gist_id
		)
	}
}

impl SlashCommand for Play {
	fn run<'a>(
		&'a self,
		helper: InteractionsHelper,
		mut responder: SlashData,
	) -> Pin<Box<dyn Future<Output = MietteResult<()>> + Send + 'a>> {
		Box::pin(async move {
			helper.ack(&responder).await.into_diagnostic()?;

			let code = ResultHandling::None.apply(&self.code.code);

			let cdn = helper.cdn();

			let request =
				PlaygroundRequest::new(&code, self.channel, self.edition, self.mode, false);

			let mut result = cdn
				.post("https://play.rust-lang.org/execute")
				.json(&request)
				.send()
				.await
				.into_diagnostic()?
				.json::<PlaygroundResponse>()
				.await
				.into_diagnostic()?;

			result.format(self.warn);

			let output = if result.stderr.is_empty() {
				result.stdout
			} else if result.stdout.is_empty() {
				result.stderr
			} else {
				format!("{}\n{}", result.stderr, result.stdout)
			};

			if output.len() > 2000 {
				let gist_id = get_gist(helper.context(), &self.code.code).await?;
				// message content too long, make it a file.
				responder.message(&format!(
					"output is too large, playground url: <{}>",
					self.url_from_gist(&gist_id)
				));
				let file = vec![AttachmentFile::from_bytes("output.txt", output.as_bytes())];
				let raw_data = serde_json::to_vec(&responder.callback).into_diagnostic()?;
				let update_message = helper
					.raw_update(&responder)
					.await?
					.attach(&file)
					.payload_json(&raw_data);

				update_message.exec().await.into_diagnostic()?;

				return Ok(());
			}

			responder.message(format!("```\n{}\n```", output));

			helper.update(&mut responder).await?;
			Ok(())
		})
	}
}

impl DefineCommand for Play {
	fn define() -> CommandBuilder {
		CommandBuilder::new(
			"play".to_owned(),
			"Runs code on the Rust Playground".to_owned(),
			CommandType::ChatInput,
		)
		.default_permission(true)
		.option(
			StringBuilder::new(
				"code".to_owned(),
				"Code to run, must be wrapped in single (`) or triple (\\`\\`\\`) backticks"
					.to_owned(),
			)
			.required(true),
		)
		.option(
			StringBuilder::new(
				"edition".to_owned(),
				"The rust edition to use (default 2018)".to_owned(),
			)
			.choices(
				vec![Edition::E2015, Edition::E2018, Edition::E2021]
					.iter()
					.copied()
					.map(single_to_tuple_of_strings),
			),
		)
		.option(
			StringBuilder::new(
				"mode".to_owned(),
				"The mode to build your code in (default debug)".to_owned(),
			)
			.choices(
				vec![BuildMode::Debug, BuildMode::Release]
					.iter()
					.copied()
					.map(single_to_tuple_of_strings),
			),
		)
		.option(
			StringBuilder::new(
				"channel".to_owned(),
				"The rust channel to use (default nightly)".to_owned(),
			)
			.choices(
				vec![RustChannel::Stable, RustChannel::Beta, RustChannel::Nightly]
					.iter()
					.copied()
					.map(single_to_tuple_of_strings),
			),
		)
		.option(BooleanBuilder::new(
			"warn".to_owned(),
			"Whether to emit warnings or not (default false)".to_owned(),
		))
	}

	fn parse(data: CommandData) -> MietteResult<Self> {
		let code_raw = data
			.options
			.clone()
			.into_iter()
			.find(|value| value.name == "code")
			.ok_or_else(|| error!("failed to find code value"))?;

		let build_mode_raw = data
			.options
			.clone()
			.into_iter()
			.find(|value| value.name == "mode");

		let edition_raw = data
			.options
			.clone()
			.into_iter()
			.find(|value| value.name == "edition");

		let channel_raw = data
			.options
			.clone()
			.into_iter()
			.find(|value| value.name == "channel");

		let warn_raw = data.options.into_iter().find(|value| value.name == "warn");

		let code = match code_raw.value {
			CommandOptionValue::String(val) => val.parse().into_diagnostic(),
			_ => Err(error!("Value is not a string")),
		}?;

		let build_mode = if let Some(value) = build_mode_raw {
			match value.value {
				CommandOptionValue::String(v) => v.parse().unwrap_or_default(),
				_ => BuildMode::default(),
			}
		} else {
			BuildMode::default()
		};

		let edition = if let Some(value) = edition_raw {
			match value.value {
				CommandOptionValue::String(v) => v.parse().unwrap_or_default(),
				_ => Edition::default(),
			}
		} else {
			Edition::default()
		};

		let channel = if let Some(value) = channel_raw {
			match value.value {
				CommandOptionValue::String(v) => v.parse().unwrap_or_default(),
				_ => RustChannel::default(),
			}
		} else {
			RustChannel::default()
		};

		let warn = if let Some(value) = warn_raw {
			match value.value {
				CommandOptionValue::Boolean(v) => v,
				_ => false,
			}
		} else {
			false
		};

		Ok(Self {
			code,
			mode: build_mode,
			edition,
			channel,
			warn,
		})
	}
}

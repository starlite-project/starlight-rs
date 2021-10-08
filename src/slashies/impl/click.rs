use super::SlashCommand;
use crate::{
	components::{BuildError, ButtonBuilder, ComponentBuilder},
	slashies::interaction::Interaction,
};
use async_trait::async_trait;
use base64::encode;
use miette::{IntoDiagnostic, Result};
use nebula::Leak;
use std::{any::type_name, collections::HashMap, lazy::Lazy, mem::MaybeUninit};
use thiserror::Error;
use twilight_gateway::Event;
use twilight_model::{
	application::{
		component::{button::ButtonStyle, Button, Component},
		interaction::{Interaction as DiscordInteraction, MessageComponentInteraction},
	},
	id::UserId,
};

pub use click_derive::*;

#[derive(Debug, Error, Clone, Copy)]
#[error("an error occurred getting data from the interaction")]
pub struct ClickError;

#[async_trait]
pub trait ClickCommand<const N: usize>: SlashCommand {
	const STYLES: [ButtonStyle; N];

	const LABELS: [&'static str; N];

	const LINKS: &'static [(usize, &'static str)] = &[];

	const COMPONENT_IDS: Lazy<[&'static str; N]> = Lazy::new(|| {
		let mut output = [""; N];

		let name = unsafe { type_name::<Self>().split("::").last().unwrap_unchecked() };

		let encoded = encode(name);

		for (i, val) in output.iter_mut().enumerate() {
			*val = (encoded.clone() + "_" + &i.to_string()).leak();
		}

		output
	});

	const EMPTY_COMPONENTS: Option<&'static [Component]> = Some(&[]);

	fn components() -> Result<Vec<Component>, BuildError> {
		Ok(vec![Self::define_buttons()?.build_component()?])
	}

	fn define_buttons() -> Result<[Button; N], BuildError> {
		let mut output: [MaybeUninit<Button>; N] = MaybeUninit::uninit_array::<N>();

		let links = Self::LINKS.iter().copied().collect::<HashMap<_, _>>();

		for (i, item) in output.iter_mut().enumerate().take(N) {
			let label = Self::LABELS[i];
			let style = Self::STYLES[i];
			let id = Self::COMPONENT_IDS[i];
			let mut button_builder = ButtonBuilder::new().label(label);

			button_builder = if let Some(link) = links.get(&i) {
				button_builder.url(link.to_owned())
			} else {
				button_builder.style(style).custom_id(id)
			};

			item.write(button_builder.build()?);
		}

		unsafe { Ok(MaybeUninit::array_assume_init(output)) }
	}

	async fn wait_for_click<'a>(
		interaction: Interaction<'a>,
		user_id: UserId,
	) -> Result<MessageComponentInteraction> {
		let message_id = interaction.get_message().await?.id;
		interaction
			.state
			.standby
			.wait_for_button(message_id, move |event: &MessageComponentInteraction| {
				event.author_id().unwrap_or_default() == user_id
			})
			.await
			.into_diagnostic()
	}

	async fn wait_for_click_old<'a>(
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
			interaction
				.state
				.standby
				.wait_for(guild_id, waiter)
				.await
				.into_diagnostic()?
		} else {
			interaction
				.state
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

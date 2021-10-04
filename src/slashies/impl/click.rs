use super::SlashCommand;
use crate::{
	components::{BuildError, ButtonBuilder, ComponentBuilder},
	slashies::interaction::Interaction,
};
use async_trait::async_trait;
use base64::encode;
use miette::{IntoDiagnostic, Result};
use nebula::Leak;
use twilight_gateway::Event;
use thiserror::Error;
use std::{any::type_name, lazy::Lazy, mem::MaybeUninit};
use twilight_model::{
	application::{
		component::{button::ButtonStyle, Button, Component},
		interaction::{MessageComponentInteraction, Interaction as DiscordInteraction},
	},
	id::UserId,
};

#[derive(Debug, Error, Clone, Copy)]
#[error("an error occurred getting data from the interaction")]
pub struct ClickError;

#[async_trait]
pub trait ClickCommand<const N: usize>: SlashCommand {
	const BUTTONS: [(&'static str, ButtonStyle); N];

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

		let component_ids = Self::COMPONENT_IDS;

		for (i, (label, style)) in Self::BUTTONS.iter().copied().enumerate() {
			output[i].write(
				ButtonBuilder::new()
					.custom_id(component_ids[i])
					.label(label)
					.style(style)
					.build()?,
			);
		}

		unsafe { Ok(MaybeUninit::array_assume_init(output)) }
	}

	async fn wait_for_click<'a>(
		interaction: Interaction<'a>,
		user_id: UserId,
	) -> Result<MessageComponentInteraction> {
        let waiter = move |event: &Event| {
            if let Event::InteractionCreate(interaction_create) = event {
                if let DiscordInteraction::MessageComponent(ref button) = interaction_create.0 {
                    if Self::COMPONENT_IDS.contains(&button.data.custom_id.as_str()) && button.author_id().unwrap_or_default() == user_id {
                        return true;
                    }
                }
            }
            false
        };

        let event = if let Some(guild_id) = interaction.command.guild_id {
            interaction.state.standby.wait_for(guild_id, waiter).await.into_diagnostic()?
        } else {
            interaction.state.standby.wait_for_event(waiter).await.into_diagnostic()?
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

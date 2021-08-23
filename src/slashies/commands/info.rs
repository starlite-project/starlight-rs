use super::SlashCommand;
use crate::{helpers::CacheHelper, slashies::Response, state::State};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use twilight_embed_builder::{EmbedAuthorBuilder, EmbedBuilder, EmbedFooterBuilder, ImageSource};
use twilight_mention::Mention;
use twilight_model::{
    application::{
        command::{BaseCommandOptionData, Command, CommandOption},
        interaction::ApplicationCommand,
    },
    user::User,
};

const GUILD_ONLY_MESSAGE: &str = "This command can only be used in a guild";
const ERROR_OCCURRED: &str = "An error occurred getting the user";

#[derive(Debug, Clone)]
pub struct Info(pub(super) ApplicationCommand);

impl Info {
    const BASE: EmbedBuilder = EmbedBuilder::new();
}

#[async_trait]
impl SlashCommand<0> for Info {
    const NAME: &'static str = "info";

    fn define() -> Command {
        Command {
            application_id: None,
            guild_id: None,
            name: String::from(Self::NAME),
            default_permission: None,
            description: String::from("Get info about a user"),
            id: None,
            options: vec![CommandOption::User(BaseCommandOptionData {
                name: String::from("user"),
                description: String::from(
                    "The user to get information about, defaulting to the author",
                ),
                required: false,
            })],
        }
    }

    async fn run(&self, state: State) -> Result<()> {
        let interaction = state.interaction(&self.0);

        let guild_id = if let Some(id) = interaction.command.guild_id {
            id
        } else {
            interaction
                .response(Response::from(GUILD_ONLY_MESSAGE))
                .await?;

            return Ok(());
        };

        let user = if let Some(resolved) = &interaction.command.data.resolved {
            resolved
                .users
                .first()
                .unwrap_or_else(|| crate::debug_unreachable!())
        } else if let Some(member) = &interaction.command.member {
            if let Some(user) = member.user.as_ref() {
                user
            } else {
                interaction
                    .response(Response::from("An error occurred getting the user"))
                    .await?;
                return Ok(());
            }
        } else {
            interaction
                .response(Response::from("An error occurred getting the user"))
                .await?;
            return Ok(());
        };

        let helper = CacheHelper::new(&interaction.state);

        let member = helper.member(guild_id, user.id).await?;

        let mut roles = helper.member_roles(guild_id, user.id).await?;

        roles.reverse();

        let mut embed_builder = Self::BASE
            .author(EmbedAuthorBuilder::new().name(format!(
                "{name}#{discriminator} - {mention}",
                name = user.name,
                discriminator = user.discriminator,
                mention = user.mention()
            )))
            .thumbnail(ImageSource::url(user_avatar(user))?)
            .footer(EmbedFooterBuilder::new(format!("ID: {}", user.id)))
            .timestamp(format!("{:?}", Utc::now()));

        let user_color = roles
            .iter()
            .map(|role| role.color)
            .find(|color| color != &0);

        embed_builder = match user_color {
            Some(color) if color != 0 => embed_builder.color(color),
            _ => embed_builder,
        };

        interaction
            .response(
                Response::from(embed_builder.build()?)
                    .build_allowed_mentions(|builder| builder.replied_user()),
            )
            .await?;

        Ok(())
    }
}

fn user_avatar(user: &User) -> String {
    if let Some(hash) = &user.avatar {
        format!(
            "https://cdn.discordapp.com/avatars/{}/{}.{}",
            user.id,
            hash,
            if hash.starts_with("a_") { "gif" } else { "png" }
        )
    } else {
        format!(
            "https://cdn.discordapp.com/embed/avatars/{}.png",
            user.discriminator
                .chars()
                .last()
                .unwrap_or_else(|| crate::debug_unreachable!())
                .to_digit(10)
                .unwrap_or_else(|| crate::debug_unreachable!())
                % 5
        )
    }
}

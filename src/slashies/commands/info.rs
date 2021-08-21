use super::SlashCommand;
use crate::{
    helpers::{cache::MemberHelper, CacheHelper},
    slashies::Response,
    state::State,
};
use anyhow::Result;
use async_trait::async_trait;
use twilight_embed_builder::{
    image_source::ImageSourceUrlError, EmbedAuthorBuilder, EmbedBuilder, ImageSource,
};
use twilight_model::{
    application::{
        command::{BaseCommandOptionData, Command, CommandOption},
        interaction::ApplicationCommand,
    },
    channel::embed::EmbedAuthor,
    user::User,
};

#[derive(Debug, Clone)]
pub struct Info(pub(super) ApplicationCommand);

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
                .response(Response::from("This command can only be used in a guild"))
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

        let embed_builder = EmbedBuilder::new().author(embed_author(&member, user)?);

        interaction
            .response(Response::from(embed_builder.build()?))
            .await?;

        Ok(())
    }
}

const fn member_name<'a>(member: &'a MemberHelper, user: &'a User) -> &'a String {
    match &member.nick {
        Some(nick) => nick,
        None => &user.name,
    }
}

fn avatar(user: &User) -> String {
    match &user.avatar {
        Some(hash) => format!(
            "https://cdn.discordapp.com/avatars/{}/{}.{}",
            user.id,
            hash,
            if hash.starts_with("a_") { "gif" } else { "png" }
        ),
        None => format!(
            "https://cdn.discordapp.com/embed/avatars/{}.png",
            user.discriminator
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap()
                % 5
        ),
    }
}

fn embed_author<'a>(
    member: &'a MemberHelper,
    user: &'a User,
) -> Result<EmbedAuthor, ImageSourceUrlError> {
    Ok(EmbedAuthorBuilder::new()
        .name(member_name(member, user))
        .icon_url(ImageSource::url(avatar(user))?)
        .build())
}

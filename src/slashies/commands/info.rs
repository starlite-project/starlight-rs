use super::{ClickCommand, SlashCommand};
use crate::{
    components::{BuildError, ButtonBuilder, ComponentBuilder},
    slashies::{interaction::Interaction, Response},
    state::State,
    InteractionAuthor,
};
use anyhow::Result;
use async_trait::async_trait;
use twilight_embed_builder::{
    image_source::ImageSourceUrlError, EmbedAuthorBuilder, EmbedBuilder, ImageSource,
};
use twilight_model::{
    application::{
        command::{BaseCommandOptionData, Command, CommandOption},
        component::{button::ButtonStyle, Button},
        interaction::ApplicationCommand,
    },
    channel::embed::EmbedAuthor,
    guild::{Member, Role},
    user::User,
};

#[derive(Debug, Clone)]
pub enum InfoType {
    Author,
    Bot,
    Guild,
}

impl InfoType {
    pub const fn is_guild(&self) -> bool {
        matches!(self, Self::Guild)
    }

    pub const fn is_bot(&self) -> bool {
        matches!(self, Self::Bot)
    }

    pub const fn is_author(&self) -> bool {
        matches!(self, Self::Author)
    }
}

#[derive(Debug, Clone)]
pub struct Info(pub(crate) ApplicationCommand);

impl<'a> Info {
    async fn run_buttons(&self, interaction: Interaction<'a>) -> Result<()> {
        let response = Response::new()
            .message("Select an option")
            .set_components(Self::components()?);

        interaction.response(response).await?;

        let click_data = Self::wait_for_click(
            interaction.state,
            interaction,
            interaction.command.interaction_author(),
        )
        .await?;

        crate::model!(interaction
            .update()?
            .content(Some(
                format!("You clicked {}", click_data.data.custom_id).as_str()
            ))?
            .components(Self::EMPTY_COMPONENTS)?);

        Ok(())
    }

    async fn run_user(&self, interaction: Interaction<'a>) -> Result<()> {
        let user = interaction
            .command
            .data
            .resolved
            .as_ref()
            .and_then(|data| data.users.get(0))
            .unwrap_or_else(|| crate::debug_unreachable!());

        let guild_id = interaction
            .command
            .guild_id
            .unwrap_or_else(|| crate::debug_unreachable!());

        let member: Member = crate::model!(interaction.state.http.guild_member(guild_id, user.id));

        let guild_roles: Vec<Role> = crate::list_models!(interaction.state.http.roles(guild_id));

        // TODO: figure out if there's a way to not make this O(n^2)
        let mut roles = guild_roles
            .iter()
            .cloned()
            .filter(|role| member.roles.contains(&role.id) && role.color != 0)
            .collect::<Vec<Role>>();

        roles.sort();

        roles.reverse();

        let highest_role = roles.get(0).unwrap_or(
            guild_roles
                .iter()
                .find(|role| role.id.0 == guild_id.0)
                .unwrap_or_else(|| crate::debug_unreachable!()),
        );

        let mut embed_builder = EmbedBuilder::new().author(embed_author(&member)?);

        if highest_role.color != 0 {
            embed_builder = embed_builder.color(highest_role.color);
        }

        let embed = embed_builder.build()?;

        let response = Response::from(embed);

        interaction.response(response).await?;

        Ok(())
    }
}

#[async_trait]
impl SlashCommand<3> for Info {
    const NAME: &'static str = "info";

    fn define() -> Command {
        Command {
            application_id: None,
            guild_id: None,
            name: String::from(Self::NAME),
            default_permission: None,
            description: String::from("Gets info about a user, the guild, or myself"),
            id: None,
            options: vec![CommandOption::User(BaseCommandOptionData {
                description: String::from("The user to get info for"),
                name: String::from("user"),
                required: false,
            })],
        }
    }

    async fn run(&self, state: State) -> Result<()> {
        let interaction = state.interaction(&self.0);

        if interaction.command.guild_id.is_none() {
            interaction
                .response(Response::from("This command can only be ran in guilds").exec())
                .await?;

            return Ok(());
        }

        if interaction.command.data.options.is_empty() {
            self.run_buttons(interaction).await
        } else {
            self.run_user(interaction).await
        }
    }
}

impl ClickCommand<3> for Info {
    type Output = InfoType;

    fn define_buttons() -> Result<[Button; 3], BuildError> {
        let component_ids = Self::COMPONENT_IDS;
        let buttons = [
            ButtonBuilder::new()
                .custom_id(component_ids[0])
                .label("Author")
                .style(ButtonStyle::Primary)
                .build()?,
            ButtonBuilder::new()
                .custom_id(component_ids[1])
                .label("Bot")
                .style(ButtonStyle::Success)
                .build()?,
            ButtonBuilder::new()
                .custom_id(component_ids[2])
                .label("Guild")
                .style(ButtonStyle::Danger)
                .build()?,
        ];

        Ok(buttons)
    }

    fn parse(_: Interaction<'_>, key: &str) -> Self::Output {
        let component_ids = Self::COMPONENT_IDS;

        let [author, bot, guild] = *component_ids;

        if key == author {
            InfoType::Author
        } else if key == bot {
            InfoType::Bot
        } else if key == guild {
            InfoType::Guild
        } else {
            crate::debug_unreachable!()
        }
    }
}

const fn member_name(member: &Member) -> &String {
    match &member.nick {
        Some(nick) => nick,
        None => &member.user.name,
    }
}

fn user_avatar(user: &User) -> String {
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

fn embed_author(member: &Member) -> Result<EmbedAuthor, ImageSourceUrlError> {
    Ok(EmbedAuthorBuilder::new()
        .name(member_name(member))
        .icon_url(ImageSource::url(user_avatar(&member.user))?)
        .build())
}

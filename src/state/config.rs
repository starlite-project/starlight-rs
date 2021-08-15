use anyhow::Result;
use clap::{crate_authors, crate_description, crate_license, crate_name, crate_version, App, Arg};
use serde::{Deserialize, Serialize};
use std::{env, ffi::OsStr, fs, path::PathBuf};
use tracing::{event, instrument, Level};
use twilight_model::id::GuildId;

const REMOVE_SLASH_COMMANDS_KEY: &str = "remove-slash-commands";
const GUILD_ID_KEY: &str = "guild-id";

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Config {
    pub guild_id: Option<GuildId>,
    pub remove_slash_commands: bool,
    pub token: &'static str,
}

impl Config {
    pub fn new() -> Result<Self> {
        let matches = App::new(crate_name!())
            .about(crate_description!())
            .author(crate_authors!())
            .license(crate_license!())
            .version(crate_version!())
            .args(&[
                Arg::new(GUILD_ID_KEY)
                    .about("Guild Id to use for testing slash commands")
                    .env("GUILD_ID")
                    .long("guild-id")
                    .takes_value(true),
                Arg::new(REMOVE_SLASH_COMMANDS_KEY)
                    .about("Removes the global slash commands and exits")
                    .env("DELETE_SLASH_COMMANDS")
                    .long("delete-slash-commands"),
            ])
            .get_matches();

        let guild_id = match matches.value_of_t::<u64>(GUILD_ID_KEY) {
            Ok(g) => Some(g.into()),
            Err(e) if e.kind == clap::ErrorKind::ArgumentNotFound => None,
            Err(e) => e.exit(),
        };

        let remove_slash_commands = matches.is_present(REMOVE_SLASH_COMMANDS_KEY);
        let token = Self::get_token()?;

        Ok(Self {
            guild_id,
            remove_slash_commands,
            token,
        })
    }

    pub fn get_user_id(self) -> Result<u64> {
        let first_part_of_token = self.token.split('.').next().unwrap_or_default();

        let decoded = base64::decode(first_part_of_token)?;

        let value = unsafe { String::from_utf8_unchecked(decoded) }.parse()?;

        Ok(value)
    }

    #[instrument]
    fn get_token() -> Result<&'static str> {
        let token = if let Some(credential_dir) = env::var_os("CREDENTIALS_DIRECTORY") {
            event!(Level::INFO, "using systemd credential storage");
            let path = [&credential_dir, OsStr::new("token")]
                .iter()
                .collect::<PathBuf>();
            fs::read_to_string(path)?
        } else {
            event!(Level::WARN, "falling back to `TOKEN` environment variabke");
            env::var("DISCORD_TOKEN")?
        };

        Ok(Box::leak(Box::new(token)))
    }
}

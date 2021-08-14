#![feature(negative_impls)]
#![warn(clippy::pedantic, clippy::nursery)]
#![deny(clippy::all)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions
)]

use anyhow::Result;
use clap::{crate_authors, crate_description, crate_license, crate_name, crate_version, App, Arg};
use std::{env, ffi::OsStr, fs, path::PathBuf};
use tracing::{event, instrument, Level};
use twilight_model::id::GuildId;

pub mod components;
pub mod ext_traits;
pub mod slashies;
pub mod state;

pub use ext_traits::GetUserId;

#[derive(Debug, Default, Clone, Copy)]
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
                Arg::new("guild-id")
                    .about("Guild Id to use for testing slash commands")
                    .env("GUILD_ID")
                    .long("guild-id")
                    .takes_value(true),
                Arg::new("remove-slash-commands")
                    .about("Removes the global slash commands and exits")
                    .env("DELETE_SLASH_COMMANDS")
                    .long("delete-slash-commands"),
            ])
            .get_matches();

        let guild_id = match matches.value_of_t::<u64>("guild-id") {
            Ok(g) => Some(g.into()),
            Err(e) if e.kind == clap::ErrorKind::ArgumentNotFound => None,
            Err(e) => e.exit(),
        };

        let remove_slash_commands = matches.is_present("remove-slash-commands");
        let token = Self::get_token()?;

        Ok(Self {
            guild_id,
            remove_slash_commands,
            token,
        })
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
            event!(Level::WARN, "falling back to `TOKEN` environment variable");
            env::var("DISCORD_TOKEN")?
        };

        Ok(Box::leak(Box::new(token)))
    }

    fn decode_user_id(token: &str) -> Result<u64> {
        let first_part_of_token = token.split('.').collect::<Vec<_>>();

        let first_part_of_token = first_part_of_token.first().unwrap();

        let decoded = base64::decode(first_part_of_token)?;

        let value = unsafe { String::from_utf8_unchecked(decoded) }.parse()?;

        Ok(value)
    }
}

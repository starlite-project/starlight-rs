use clap::{crate_authors, crate_description, crate_license, crate_name, crate_version, App, Arg};
use std::{env, ffi::OsStr, fs, path::PathBuf};
use tracing::{event, instrument, Level};
use twilight_model::id::GuildId;

pub mod slashies;
pub mod state;

pub type GenericResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[instrument]
pub fn token() -> GenericResult<String> {
    let token = if let Some(credential_dir) = env::var_os("CREDENTIALS_DIRECTORY") {
        event!(Level::INFO, "using systemd credential storage");
        let path: PathBuf = [&credential_dir, OsStr::new("token")].iter().collect();
        fs::read_to_string(path)?
    } else {
        event!(Level::WARN, "falling back to `TOKEN` environment variable");
        env::var("DISCORD_TOKEN")?
    };

    Ok(token)
}

pub fn config() -> GenericResult<Config> {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .license(crate_license!())
        .version(crate_version!())
        .args(&[
            Arg::new("guild_id")
                .about("Guild Id to use for testing slash commands")
                .env("TESTING_GUILD_ID")
                .long("guild-id")
                .takes_value(true),
            Arg::new("remove-slash-commands")
                .about("Removes the global slash commands and exits")
                .env("DELETE_SLASH_COMMANDS")
                .long("delete-slash-commands"),
        ]);
}

pub struct Config {
    pub guild_id: Option<GuildId>,
    pub remove_slash_commands: bool,
    pub token: String,
}

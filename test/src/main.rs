use serde::{Deserialize, Serialize};
use twilight_model::application::command::{Command, CommandOption};

pub type StarSlashies = Vec<StarCommand>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarCommand {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_permissions: Option<bool>,
    pub description: String,
    #[serde(default)]
    pub options: Vec<CommandOption>,
}

impl PartialEq<Command> for StarCommand {
    fn eq(&self, other: &Command) -> bool {
        (
            &self.name,
            self.default_permissions,
            &self.description,
            &self.options,
        ) == (
            &other.name,
            other.default_permission,
            &other.description,
            &other.options,
        )
    }
}

impl From<StarCommand> for Command {
    fn from(value: StarCommand) -> Self {
        Self {
            application_id: None,
            guild_id: None,
            name: value.name,
            default_permission: value.default_permissions,
            description: value.description,
            id: None,
            options: value.options,
        }
    }
}

impl From<&StarCommand> for Command {
    fn from(value: &StarCommand) -> Self {
        Self {
            application_id: None,
            guild_id: None,
            name: value.name.clone(),
            default_permission: value.default_permissions,
            description: value.description.clone(),
            id: None,
            options: value.options.clone(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("./commands.json");

    let json = std::fs::read_to_string(path)?;

    let val: StarSlashies = serde_json::from_str(&json)?;

    dbg!(&val);

    dbg!(&val.iter().map(Command::from).collect::<Vec<_>>());

    Ok(())
}

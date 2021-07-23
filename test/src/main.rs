use serde::{Deserialize, Serialize};
use twilight_model::application::command::{
    BaseCommandOptionData, ChoiceCommandOptionData, Command, CommandOption,
};

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
    fn from(value: StarCommand) -> Command {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let val = StarCommand {
        name: "ping".to_owned(),
        default_permissions: None,
        description: "Pings the bot".to_owned(),
        options: vec![],
    };

    let json = serde_json::to_string_pretty(&val)?;

    eprintln!("{}", &json);

    let val: StarCommand = serde_json::from_str(&json)?;

    let cmd: Command = val.clone().into();

    dbg!(val);

    dbg!(cmd);

    Ok(())
}

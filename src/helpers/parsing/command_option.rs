use twilight_model::{
	application::interaction::application_command::{CommandDataOption, CommandOptionValue},
	id::{
		marker::{ChannelMarker, GenericMarker, RoleMarker, UserMarker},
		Id,
	},
};

pub trait CommandParse<T> {
	// to avoid naming conflicts with anything else that could use parse..
	fn parse_option(self) -> Option<T>;
}

// boolean
impl CommandParse<bool> for CommandOptionValue {
	fn parse_option(self) -> Option<bool> {
		if let Self::Boolean(b) = self {
			Some(b)
		} else {
			None
		}
	}
}

// channel
impl CommandParse<Id<ChannelMarker>> for CommandOptionValue {
	fn parse_option(self) -> Option<Id<ChannelMarker>> {
		if let Self::Channel(id) = self {
			Some(id)
		} else {
			None
		}
	}
}

// integer
impl CommandParse<i64> for CommandOptionValue {
	fn parse_option(self) -> Option<i64> {
		if let Self::Integer(v) = self {
			Some(v)
		} else {
			None
		}
	}
}

// mentionable
impl CommandParse<Id<GenericMarker>> for CommandOptionValue {
	fn parse_option(self) -> Option<Id<GenericMarker>> {
		if let Self::Mentionable(id) = self {
			Some(id)
		} else {
			None
		}
	}
}

// number
impl CommandParse<f64> for CommandOptionValue {
	fn parse_option(self) -> Option<f64> {
		if let Self::Number(n) = self {
			Some(n.0)
		} else {
			None
		}
	}
}

// role
impl CommandParse<Id<RoleMarker>> for CommandOptionValue {
	fn parse_option(self) -> Option<Id<RoleMarker>> {
		if let Self::Role(id) = self {
			Some(id)
		} else {
			None
		}
	}
}

// string
impl CommandParse<String> for CommandOptionValue {
	fn parse_option(self) -> Option<String> {
		if let Self::String(v) = self {
			Some(v)
		} else {
			None
		}
	}
}

// subcommand
#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn parse_subcommand(value: CommandOptionValue) -> Option<Vec<CommandDataOption>> {
	if let CommandOptionValue::SubCommand(v) = value {
		Some(v)
	} else {
		None
	}
}

// subcommand group
#[allow(clippy::missing_const_for_fn)]
#[must_use]
pub fn parse_subcommand_group(value: CommandOptionValue) -> Option<Vec<CommandDataOption>> {
	if let CommandOptionValue::SubCommandGroup(v) = value {
		Some(v)
	} else {
		None
	}
}

// user
impl CommandParse<Id<UserMarker>> for CommandOptionValue {
	fn parse_option(self) -> Option<Id<UserMarker>> {
		if let Self::User(id) = self {
			Some(id)
		} else {
			None
		}
	}
}

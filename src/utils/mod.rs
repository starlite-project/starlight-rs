use once_cell::sync::Lazy;
#[cfg(feature = "docker")]
use std::{io::Error as IoError, net::ToSocketAddrs};
use sysinfo::{get_current_pid, Process, System, SystemExt};
use thiserror::Error;
use twilight_cache_inmemory::ResourceType;
use twilight_model::{application::interaction::ApplicationCommand, id::UserId};

pub mod constants;

#[derive(Debug, Error)]
#[allow(missing_copy_implementations)]
pub enum UtilError {
	#[error("an error occurred getting the pid")]
	Pid,
	#[error("an error occurred getting the current process")]
	Process,
	#[cfg(feature = "docker")]
	#[cfg_attr(feature = "docker", error("an error occurred getting the socket"))]
	Address(#[from] IoError),
	#[error("the {0} was an option value that was None")]
	OptionWasNone(&'static str),
}

static mut SYSTEM: Lazy<System> = Lazy::new(System::new);

pub fn get_current_process<'a>() -> Result<&'a Process, UtilError> {
	let process_id = get_current_pid().map_err(|_| UtilError::Pid)?;
	unsafe {
		if SYSTEM.refresh_process(process_id) {
			SYSTEM.process(process_id).ok_or(UtilError::Process)
		} else {
			Err(UtilError::Process)
		}
	}
}

#[must_use]
pub const fn interaction_author(command: &ApplicationCommand) -> UserId {
	if let Some(ref member) = command.member {
		if let Some(user) = &member.user {
			return user.id;
		}
	}

	if let Some(ref user) = command.user {
		return user.id;
	}

	unsafe { UserId::new_unchecked(1) }
}

#[cfg(feature = "docker")]
pub fn get_host(host: &str, port: u16) -> Result<String, UtilError> {
	(host, port)
		.to_socket_addrs()?
		.next()
		.ok_or(UtilError::OptionWasNone("socket"))
		.map(|socket| socket.to_string())
}

pub trait CacheReliant {
	fn needs() -> ResourceType;
}

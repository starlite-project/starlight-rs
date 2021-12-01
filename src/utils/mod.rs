#[cfg(feature = "docker")]
use std::{io::Error as IoError, net::ToSocketAddrs};

use crate::prelude::*;

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

#[cfg(feature = "docker")]
pub fn get_host(host: &str, port: u16) -> Result<String, UtilError> {
	(host, port)
		.to_socket_addrs()?
		.next()
		.ok_or(UtilError::OptionWasNone("socket"))
		.map(|socket| socket.to_string())
}

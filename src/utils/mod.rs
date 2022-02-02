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

#[must_use]
pub fn levenshtein(a: &str, b: &str) -> usize {
	let mut result = 0;

	if a == b {
		return result;
	}

	let length_a = a.chars().count();
	let length_b = b.chars().count();

	if length_a == 0 {
		return length_b;
	}

	if length_b == 0 {
		return length_a;
	}

	let mut cache = (1..).take(length_a).collect::<Vec<usize>>();
	let mut distance_a;
	let mut distance_b;

	for (index_b, code_b) in b.chars().enumerate() {
		result = index_b;
		distance_a = index_b;

		for (index_a, code_a) in a.chars().enumerate() {
			distance_b = if code_a == code_b {
				distance_a
			} else {
				distance_a + 1
			};

			distance_a = cache[index_a];

			result = if distance_a > result {
				if distance_b > result {
					result + 1
				} else {
					distance_b
				}
			} else if distance_b > distance_a {
				distance_a + 1
			} else {
				distance_b
			};

			cache[index_a] = result;
		}
	}

	result
}

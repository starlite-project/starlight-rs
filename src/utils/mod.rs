use once_cell::sync::Lazy;
use std::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
};
use sysinfo::{get_current_pid, Process, System, SystemExt};

pub mod constants;

#[derive(Debug)]
pub struct UtilError {
	source: Option<Box<dyn Error + Send + Sync>>,
	kind: UtilErrorType,
}

impl UtilError {
	#[must_use]
	pub const fn kind(&self) -> UtilErrorType {
		self.kind
	}

	#[must_use]
	pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
		self.source
	}

	#[must_use]
	pub fn into_parts(self) -> (UtilErrorType, Option<Box<dyn Error + Send + Sync>>) {
		(self.kind, self.source)
	}

	fn pid(_: &str) -> Self {
		Self {
			source: None,
			kind: UtilErrorType::PidError,
		}
	}

	fn process() -> Self {
		Self {
			source: None,
			kind: UtilErrorType::ProcessError,
		}
	}
}

impl Display for UtilError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self.kind {
			UtilErrorType::PidError => f.write_str("an error occurred getting the pid"),
			UtilErrorType::ProcessError => {
				f.write_str("an error occurred getting the current process")
			}
		}
	}
}

impl Error for UtilError {}

#[derive(Debug, Clone, Copy)]
pub enum UtilErrorType {
	PidError,
	ProcessError,
}

static mut SYSTEM: Lazy<System> = Lazy::new(System::new);

pub fn get_current_process<'a>() -> Result<&'a Process, UtilError> {
	let process_id = get_current_pid().map_err(UtilError::pid)?;
	unsafe {
		if SYSTEM.refresh_process(process_id) {
			SYSTEM.process(process_id).ok_or_else(UtilError::process)
		} else {
			Err(UtilError::process())
		}
	}
}

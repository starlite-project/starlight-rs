use miette::Result;
use std::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug, Clone, Copy)]
struct TestError;

impl Display for TestError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("This is a test")
	}
}

impl Error for TestError {}

fn this_fails() -> Result<()> {
	miette::bail!(TestError)
}

fn main() -> Result<()> {
	this_fails()?;

	Ok(())
}

use super::ClickCommand;
use crate::slashies::interaction::Interaction;
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;
use twilight_model::application::component::Button;

#[derive(Debug, Error, Clone)]
pub enum ParseError {
	#[error(transparent)]
	Int(#[from] ParseIntError),
	#[error(transparent)]
	Float(#[from] ParseFloatError),
	#[error("{0}")]
	Custom(&'static str),
}

pub trait ParseCommand {
	type Output;

	fn parse(interaction: Interaction, input: &str) -> Result<Self::Output, ParseError>;

	fn parse_u8(input: &str) -> Result<u8, ParseError> {
		Ok(input.parse()?)
	}

	fn parse_u16(input: &str) -> Result<u16, ParseError> {
		Ok(input.parse()?)
	}

	fn parse_u32(input: &str) -> Result<u32, ParseError> {
		Ok(input.parse()?)
	}

	fn parse_u64(input: &str) -> Result<u64, ParseError> {
		Ok(input.parse()?)
	}

	fn parse_u128(input: &str) -> Result<u128, ParseError> {
		Ok(input.parse()?)
	}

	fn parse_usize(input: &str) -> Result<usize, ParseError> {
		Ok(input.parse()?)
	}

	fn parse_i8(input: &str) -> Result<i8, ParseError> {
		Ok(input.parse()?)
	}

	fn parse_i16(input: &str) -> Result<i16, ParseError> {
		Ok(input.parse()?)
	}

	fn parse_i32(input: &str) -> Result<i32, ParseError> {
		Ok(input.parse()?)
	}

	fn parse_i64(input: &str) -> Result<i64, ParseError> {
		Ok(input.parse()?)
	}

	fn parse_i128(input: &str) -> Result<i128, ParseError> {
		Ok(input.parse()?)
	}

	fn parse_isize(input: &str) -> Result<isize, ParseError> {
		Ok(input.parse()?)
	}

	fn parse_f32(input: &str) -> Result<f32, ParseError> {
		let parsed = input.parse::<f32>()?;

		if parsed.is_infinite() {
			return Err(ParseError::Custom("expected a finite number"));
		}

		if parsed.is_nan() {
			return Err(ParseError::Custom("expected a valid number"));
		}

		Ok(parsed)
	}

	fn parse_f64(input: &str) -> Result<f64, ParseError> {
		let parsed = input.parse::<f64>()?;

		if parsed.is_infinite() {
			return Err(ParseError::Custom("expected a finite number"));
		}

		if parsed.is_nan() {
			return Err(ParseError::Custom("expected a valid number"));
		}

		Ok(parsed)
	}

	fn parse_many(
		interaction: Interaction,
		input: &[&str],
	) -> Result<Vec<Self::Output>, ParseError> {
		let mut output = Vec::with_capacity(input.len());

		for item in input {
			output.push(Self::parse(interaction, item)?);
		}

		Ok(output)
	}

	fn parse_button<const N: usize>(input: &str) -> Result<Button, ParseError>
	where
		Self: ClickCommand<N>,
	{
		let buttons = Self::define_buttons()
			.map_err(|_| ParseError::Custom("could not get button components"))?;

		buttons
			.iter()
			.cloned()
			.find(|button| button.custom_id.as_deref() == Some(input))
			.ok_or(ParseError::Custom(
				"an error occurred while getting the button pressed (this shouldn't happen)",
			))
	}
}

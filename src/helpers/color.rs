use std::fmt::{Formatter, Result as FmtResult};

use serde::{
	de::{Error as DeError, Visitor},
	Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use = "Color has no side effects"]
pub struct Color(u8, u8, u8);

impl Color {
	pub const fn new(r: u8, g: u8, b: u8) -> Self {
		Self(r, g, b)
	}

	#[must_use]
	pub const fn r(self) -> u8 {
		self.0
	}

	#[must_use]
	pub const fn g(self) -> u8 {
		self.1
	}

	#[must_use]
	pub const fn b(self) -> u8 {
		self.2
	}

	#[must_use]
	pub const fn to_decimal(self) -> u32 {
		let r = self.r() as u32;
		let g = self.g() as u32;
		let b = self.b() as u32;

		(r << 16) + (g << 8) + b
	}

	#[allow(clippy::cast_possible_truncation)]
	pub const fn from_decimal(decimal: u32) -> Self {
		let r = ((decimal & 0x00ff_0000) >> 16) as u8;
		let g = ((decimal & 0x0000_ff00) >> 8) as u8;
		let b = (decimal & 0x0000_00ff) as u8;

		Self(r, g, b)
	}
}

impl Default for Color {
	fn default() -> Self {
		Self(255, 255, 255)
	}
}

impl Serialize for Color {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_u32(self.to_decimal())
	}
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
	type Value = Color;

	fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
		formatter.write_str("a valid u32")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: DeError,
	{
		Ok(Color::from_decimal(v.parse().map_err(DeError::custom)?))
	}

	fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
	where
		E: DeError,
	{
		Ok(Color::from_decimal(v.into()))
	}

	fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
	where
		E: DeError,
	{
		Ok(Color::from_decimal(v.into()))
	}

	fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
	where
		E: DeError,
	{
		Ok(Color::from_decimal(v))
	}
}

impl<'de> Deserialize<'de> for Color {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_u32(ColorVisitor)
	}
}

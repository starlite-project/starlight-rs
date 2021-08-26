pub mod cache;
pub mod color;

pub use self::{
	cache::{models, CacheHelper},
	color::Color,
};

pub const STARLIGHT_PRIMARY_COLOR: Color = Color::new(132, 61, 164);
pub const STARLIGHT_SECONDARY_COLOR: Color = Color::new(218, 0, 78);

#[cfg(test)]
mod tests {
	use super::{STARLIGHT_PRIMARY_COLOR, STARLIGHT_SECONDARY_COLOR};

	#[test]
	fn primary_color() {
		assert_eq!(STARLIGHT_PRIMARY_COLOR.to_decimal(), 14286926)
	}
}

use syn::{Attribute, Error, LitInt, LitStr, Result, TypeSlice, parse::{Nothing, Parse, ParseStream, Parser}, parse_macro_input};
use twilight_model::application::component::button::ButtonStyle;

pub struct ClickAttributes {
	pub buttons: Buttons,
	pub styles: Styles,
}

#[derive(Debug, Clone, Copy)]
pub struct Buttons(pub usize);

impl Parse for Buttons {
	fn parse(input: ParseStream) -> Result<Self> {
		let int: LitInt = input.parse()?;
		let value = int.base10_parse::<usize>()?;
		Ok(Self(value))
	}
}

#[derive(Debug, Clone)]
pub struct Styles(Vec<&'static str>);

impl Parse for Styles {
    fn parse(input: ParseStream) -> Result<Self> {
        let first: LitStr = input.parse()?;
        
        Ok(Self(vec![Box::leak(Box::new(first.value()))]))
    }
}

#[derive(Default)]
struct ClickAttributesBuilder<'a> {
	buttons: Option<&'a Attribute>,
    labels: Option<&'a Attribute>,
	styles: Option<&'a Attribute>,
}

impl<'a> ClickAttributesBuilder<'a> {
	fn build(self) -> Result<ClickAttributes> {
		if self.buttons.is_none() {
			panic!("expected buttons attribute");
		}

		if self.styles.is_none() {
			panic!("expected styles attribute");
		}

		Ok(ClickAttributes {
			buttons: self.buttons.unwrap().parse_args()?,
			styles: self.styles.unwrap().parse_args()?,
		})
	}
}

pub fn get(input: &[Attribute]) -> Result<ClickAttributes> {
	let mut builder = ClickAttributesBuilder::default();

	for attr in input {
		if attr.path.is_ident("buttons") {
			require_non_empty_attribute(attr, "buttons")?;
			if builder.buttons.is_some() {
				return Err(Error::new_spanned(attr, "duplicate #[buttons] attribute"));
			}
			builder.buttons = Some(attr);
		} else if attr.path.is_ident("styles") {
			require_non_empty_attribute(attr, "styles")?;
			if builder.styles.is_some() {
				return Err(Error::new_spanned(attr, "duplicate #[styles] attribute"));
			}
			builder.styles = Some(attr);
		}
	}

	builder.build()
}

fn require_non_empty_attribute(attr: &Attribute, name: &str) -> Result<()> {
	match syn::parse2::<Nothing>(attr.tokens.clone()) {
		Ok(_) => Err(Error::new_spanned(
			attr,
			format!("expected tokens with attribute {}", name),
		)),
		Err(_) => Ok(()),
	}
}

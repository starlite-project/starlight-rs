use proc_macro2::{Punct, Spacing, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
	parse::{Parse, ParseStream},
	punctuated::Punctuated,
	Error, Ident, LitStr, Result, Token,
};

#[derive(Debug, Clone)]
pub struct Labels(pub Vec<LitStr>);

impl Parse for Labels {
	fn parse(input: ParseStream) -> Result<Self> {
		let list = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?;

		let inner: Vec<LitStr> = list.into_iter().collect();

		let failing = inner
			.iter()
			.cloned()
			.find(|lit_str| lit_str.value().is_empty());

		match failing {
			None => Ok(Self(inner)),
			Some(failing_string) => Err(Error::new(
				failing_string.span(),
				"expected non empty string",
			)),
		}
	}
}

impl ToTokens for Labels {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let mut parsed = Vec::with_capacity(self.0.len() * 2);
		for lit_str in self.0.clone().into_iter() {
			parsed.push(lit_str.to_token_stream());
			parsed.push(Punct::new(',', Spacing::Alone).to_token_stream());
		}

		tokens.extend(parsed);
	}
}

#[derive(Debug, Clone)]
pub struct Styles(pub Vec<Ident>);

impl Parse for Styles {
	fn parse(input: ParseStream) -> Result<Self> {
		let list = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;

		let inner: Vec<Ident> = list.into_iter().collect();

		let failing = inner.iter().cloned().find(|ident| {
			ident != "Success" && ident != "Primary" && ident != "Secondary" && ident != "Danger"
		});

		match failing {
			None => Ok(Self(inner)),
			Some(failing_ident) => Err(Error::new(
				failing_ident.span(),
				format!(
					"expected one of `Primary`, `Secondary`, `Danger`, `Success`, got {}",
					&failing_ident
				),
			)),
		}
	}
}

impl ToTokens for Styles {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let mut parsed = Vec::with_capacity(self.0.len() * 2);

		let base_path = quote!(twilight_model::application::component::button::ButtonStyle);
		let base_colons = {
			let mut vec = Vec::with_capacity(3);

			vec.push(TokenTree::from(Punct::new(':', Spacing::Joint)));
			vec.push(TokenTree::from(Punct::new(':', Spacing::Joint)));
			vec
		};

		for value in self.0.clone().into_iter() {
			let path = {
				let mut base = base_path.clone();

				let mut colons = base_colons.clone();
				colons.push(TokenTree::from(value));
				base.extend(colons);

				base
			};
			parsed.push(path);
			parsed.push(TokenTree::from(Punct::new(',', Spacing::Alone)).into());
		}

		tokens.extend(parsed);
	}
}

use std::collections::HashMap;

use proc_macro2::{Delimiter, Group, Literal, Punct, Spacing, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
	parenthesized,
	parse::{Parse, ParseStream},
	punctuated::Punctuated,
	Error, Ident, LitStr, Result, Token,
};
use url::Url;

#[derive(Debug, Clone)]
pub struct Buttons {
	pub labels: Labels,
	pub styles: Styles,
	pub links: Links,
	pub size: usize,
}

impl Parse for Buttons {
	fn parse(input: ParseStream) -> Result<Self> {
		let parsed = Punctuated::<Click, Token![,]>::parse_terminated(input)?;

		let mut styles = Vec::with_capacity(parsed.len());
		let mut labels = Vec::with_capacity(parsed.len());
		let mut links = HashMap::new();
		let mut size = 0;

		for (i, button) in parsed.into_iter().enumerate() {
			size += 1;
			styles.push(button.0);
			labels.push(button.1);
			if let Some(url) = button.2 {
				links.insert(i, url);
			}
		}

		if labels.len() != size {
			return Err(Error::new(
				input.span(),
				"labels we're an invalid length (this shouldn't happen)",
			));
		}

		if styles.len() != size {
			return Err(Error::new(
				input.span(),
				"styles we're an invalid length (this shouldn't happen)",
			));
		}

		if links.len() > size || links.len() > labels.len() || links.len() > styles.len() {
			return Err(Error::new(
				input.span(),
				"links we're an invalid length (this shouldn't happen)",
			));
		}

		Ok(Self {
			labels: Labels(labels),
			styles: Styles(styles),
			links: Links(links),
			size,
		})
	}
}

#[derive(Debug, Clone)]
pub struct Click(ButtonStyle, LitStr, Option<Link>);

impl Parse for Click {
	fn parse(input: ParseStream) -> Result<Self> {
		let style = input.parse::<ButtonStyle>()?;

		let rest;
		parenthesized!(rest in input);

		let label = rest.parse::<LitStr>()?;

		let link = if style.is_link() {
			rest.parse::<Punct>()?;
			Some(rest.parse()?)
		} else {
			None
		};

		Ok(Self(style, label, link))
	}
}

#[derive(Debug, Clone)]
pub struct ButtonStyle(Ident);

impl ButtonStyle {
	fn is_link(&self) -> bool {
		self.0 == "Link"
	}
}

impl Parse for ButtonStyle {
	fn parse(input: ParseStream) -> Result<Self> {
		let ident = input.parse::<Ident>()?;

		if ident == "Success"
			|| ident == "Danger"
			|| ident == "Primary"
			|| ident == "Secondary"
			|| ident == "Link"
		{
			Ok(Self(ident))
		} else {
			Err(Error::new_spanned(ident.clone(), format!("expected style to be `Success`, `Danger`, `Primary`, `Secondary`, or `Link`, got '{}'", ident)))
		}
	}
}

#[derive(Debug, Clone)]
pub struct Link(LitStr);

impl Parse for Link {
	fn parse(input: ParseStream) -> Result<Self> {
		let inner = input.parse::<LitStr>()?;

		if let Err(error) = Url::parse(inner.value().as_str()) {
			Err(Error::new_spanned(
				inner,
				format!("expected valid url, reason: {}", &error),
			))
		} else {
			Ok(Self(inner))
		}
	}
}

#[derive(Debug, Clone)]
pub struct Labels(pub Vec<LitStr>);

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
pub struct Styles(pub Vec<ButtonStyle>);

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
				colons.push(TokenTree::from(value.0));
				base.extend(colons);

				base
			};
			parsed.push(path);
			parsed.push(TokenTree::from(Punct::new(',', Spacing::Alone)).into());
		}

		tokens.extend(parsed);
	}
}

#[derive(Debug, Clone)]
pub struct Links(HashMap<usize, Link>);

impl ToTokens for Links {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let mut parsed = Vec::with_capacity(self.0.len());
		let base_stream = TokenStream::new();
		let comma = Punct::new(',', Spacing::Alone);

		for (i, link) in self.0.clone().into_iter() {
			let mut stream = base_stream.clone();

			let index_tree = TokenTree::Literal(Literal::usize_suffixed(i));
			let url_literal = TokenTree::Literal(Literal::string(&link.0.value()));
			stream.extend(vec![
				index_tree,
				TokenTree::Punct(comma.clone()),
				url_literal,
			]);
			parsed.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, stream)));
		}

		tokens.extend(parsed);
	}
}

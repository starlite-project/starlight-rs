#![feature(option_result_unwrap_unchecked)]

mod attr;

extern crate proc_macro;

use crate::attr::Buttons;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use supernova::cloned;
use syn::{parse_macro_input, DeriveInput, Error, Result};

#[proc_macro_derive(ClickCommand, attributes(buttons))]
pub fn derive_click(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	parse(input)
		.unwrap_or_else(|err| err.to_compile_error())
		.into()
}

fn parse(input: DeriveInput) -> Result<TokenStream2> {
	let name = &input.ident;

	let buttons: Buttons = input
		.attrs
		.iter()
		.find(|attr| attr.path.is_ident("buttons"))
		.ok_or_else(cloned!(input => || Error::new_spanned(input, "expected buttons attribute")))?
		.parse_args()?;

	let labels = buttons.labels;

	let styles = buttons.styles;

	let links = buttons.links;

	let size = buttons.size;

	if size > 5 {
		panic!("cannot have more than 5 buttons")
	}

	let tokens = quote! {
		#[automatically_derived]
		impl ClickCommand<#size> for #name {
			const LABELS: [&'static str; #size] = [#labels];

			const STYLES: [twilight_model::application::component::button::ButtonStyle; #size] = [#styles];

			const LINKS: &'static [(usize, &'static str)] = &[#links];
		}
	};

	Ok(tokens)
}

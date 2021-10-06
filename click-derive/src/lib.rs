#![feature(option_result_unwrap_unchecked)]

mod attr;

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Result};

use crate::attr::{Labels, Styles};

#[proc_macro_derive(ClickCommand, attributes(styles, labels))]
pub fn derive_click(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	parse(input)
		.unwrap_or_else(|err| err.to_compile_error())
		.into()
}

fn parse(input: DeriveInput) -> Result<TokenStream2> {
	let name = &input.ident;

	let styles: Styles = input
		.attrs
		.iter()
		.find(|value| value.path.is_ident("styles"))
		.unwrap_or_else(|| panic!("expected styles attribute"))
		.parse_args()?;
	let labels: Labels = input
		.attrs
		.iter()
		.find(|value| value.path.is_ident("labels"))
		.unwrap_or_else(|| panic!("expected labels attribute"))
		.parse_args()?;

	if labels.0.len() != styles.0.len() {
		panic!("expected equal labels and styles");
	}

	let buttons = labels.0.len();

	let tokens = quote! {
		#[automatically_derived]
		impl ClickCommand<#buttons> for #name {
			const LABELS: [&'static str; #buttons] = [#labels];

			const STYLES: [twilight_model::application::component::button::ButtonStyle; #buttons] = [#styles];
		}
	};

	Ok(tokens)
}

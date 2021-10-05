#![feature(option_result_unwrap_unchecked)]

mod attr;
mod util;

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Result};

#[proc_macro_derive(ClickCommand, attributes(styles, labels))]
pub fn derive_click(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	parse(input)
		.unwrap_or_else(|err| err.to_compile_error())
		.into()
}

fn parse(input: DeriveInput) -> Result<TokenStream2> {
	let name = &input.ident;

	let attributes = attr::get(&input)?;
	let labels = attributes.labels;
	let styles = attributes.styles;

	if labels.0.len() != styles.0.len() {
		panic!("expected equal labels and styles");
	}

	let buttons = labels.0.len();

	let tokens = quote! {
		impl ClickCommand<#buttons> for #name {
			const LABELS: [&'static str; #buttons] = [#labels];

			const STYLES: [twilight_model::application::component::button::ButtonStyle; #buttons] = [#styles];
		}
	};

	Ok(tokens)
}
